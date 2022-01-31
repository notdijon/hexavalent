use std::any::Any;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::os::raw::c_int;
use std::panic::{catch_unwind, UnwindSafe};
use std::process;
use std::ptr;
use std::ptr::NonNull;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::usize;

use crate::ffi::{hexchat_plugin, result_to_int, RawPluginHandle};
use crate::plugin::{Plugin, PluginHandle};

/// Plugin handle used to log caught panics, when the normal (safe) plugin context might not be available.
static LAST_RESORT_PLUGIN_HANDLE: AtomicPtr<hexchat_plugin> = AtomicPtr::new(ptr::null_mut());

/// Runs a closure under `catch_unwind` and logs the panic using `hexchat_print` if it happens.
///
/// Warning: if `LAST_RESORT_PLUGIN_HANDLE` is not defined when a panic happens, this function will abort the process.
pub(crate) fn catch_and_log_unwind<R>(
    ctxt_msg: &str,
    f: impl FnOnce() -> R + UnwindSafe,
) -> Result<R, ()> {
    fn abort_process_due_to_panic_in_panic_logger() -> ! {
        process::abort()
    }

    fn abort_process_due_to_panic_without_plugin_handle() -> ! {
        process::abort()
    }

    catch_unwind(|| match catch_unwind(f) {
        Ok(x) => Ok(x),
        Err(e) => {
            let panic_msg = if let Some(s) = e.downcast_ref::<String>() {
                s.as_str()
            } else if let Some(s) = e.downcast_ref::<&'static str>() {
                s
            } else {
                &"<unknown>"
            };

            eprintln!(
                "WARNING: `hexavalent` caught panic (in `{}`): {}",
                ctxt_msg, panic_msg
            );

            let plugin_handle = LAST_RESORT_PLUGIN_HANDLE.load(Ordering::Relaxed);
            if plugin_handle.is_null() {
                eprintln!("FATAL: `hexavalent` cannot find a plugin context");
                abort_process_due_to_panic_without_plugin_handle()
            } else {
                let message = format!(
                    "WARNING: `hexavalent` caught panic (in `{}`): {}\0",
                    ctxt_msg, panic_msg
                );
                // Safety: message is null-terminated
                // (Un)Safety: plugin_handle may not be valid, but there's nothing we can do here
                unsafe { ((*plugin_handle).hexchat_print)(plugin_handle, message.as_ptr().cast()) }
                Err(())
            }
        }
    })
    .unwrap_or_else(|_| abort_process_due_to_panic_in_panic_logger())
}

const NO_READERS: usize = 0;
const LOCKED: usize = usize::MAX;

/// Keeps track of the number of `with_plugin_state` invocations on the stack at once.
///
/// `usize::MAX` means that references are being updated by `hexchat_plugin_init` or `hexchat_plugin_deinit`,
/// and it is unsafe to create new references.
///
/// Similar to an RWLock, but used only to validate that HexChat is behaving safely.
/// That is, if a function in this module encounters a "locked" state, it panics instead of blocking.
static STATE: AtomicUsize = AtomicUsize::new(NO_READERS);

/// Container for types externally synchronized by `STATE`.
struct ExtSync<T>(UnsafeCell<T>);

// This impl is only sound if HexChat always invokes us from the same thread (the library-wide safety assumption).
unsafe impl<T> Sync for ExtSync<T> {}

impl<T> Deref for ExtSync<T> {
    type Target = UnsafeCell<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct GlobalPlugin {
    #[cfg(debug_assertions)]
    thread_id: std::thread::ThreadId,
    plugin: Box<dyn Any>,
    plugin_handle: NonNull<hexchat_plugin>,
}

/// Global handle to the user's plugin data and the global HexChat plugin context.
///
/// Only accessible outside this module via the safe interface `with_plugin_state`.
static PLUGIN: ExtSync<Option<GlobalPlugin>> = ExtSync(UnsafeCell::new(None));

/// Initializes a plugin of type `P`.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
pub(crate) unsafe fn hexchat_plugin_init<P: Plugin>(plugin_handle: *mut hexchat_plugin) -> c_int {
    result_to_int(catch_and_log_unwind("init", || {
        LAST_RESORT_PLUGIN_HANDLE.store(plugin_handle, Ordering::Relaxed);

        let plugin_handle = match NonNull::new(plugin_handle) {
            Some(ph) => ph,
            None => panic!("Plugin initialized with null handle"),
        };

        {
            STATE
                .compare_exchange(NO_READERS, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
                .unwrap_or_else(|e| panic!("Plugin initialized while running, state: {}", e));
            defer! { STATE.store(NO_READERS, Ordering::Relaxed) };

            // Safety: STATE guarantees unique access to handles
            unsafe {
                *PLUGIN.get() = Some(GlobalPlugin {
                    #[cfg(debug_assertions)]
                    thread_id: std::thread::current().id(),
                    plugin: Box::new(P::default()),
                    plugin_handle,
                });
            }
        }

        with_plugin_state(|plugin: &P, ph| plugin.init(ph));
    }))
}

/// Deinitializes a plugin of type `P`.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
pub(crate) unsafe fn hexchat_plugin_deinit<P: Plugin>(plugin_handle: *mut hexchat_plugin) -> c_int {
    let _ = plugin_handle;
    result_to_int(catch_and_log_unwind("deinit", || {
        with_plugin_state(|plugin: &P, ph| plugin.deinit(ph));

        {
            STATE
                .compare_exchange(NO_READERS, LOCKED, Ordering::Relaxed, Ordering::Relaxed)
                .unwrap_or_else(|e| panic!("Plugin deinitialized while running, state: {}", e));
            defer! { STATE.store(NO_READERS, Ordering::Relaxed) };

            // Safety: STATE guarantees unique access to handles
            unsafe {
                *PLUGIN.get() = None;
            }
        }

        LAST_RESORT_PLUGIN_HANDLE.store(ptr::null_mut(), Ordering::Relaxed);
    }))
}

/// Gets a safe reference to the current HexChat plugin handle and a plugin of type `P`.
///
/// # Panics
///
/// If the plugin is not initialized.
///
/// If the plugin is currently being initialized or deinitialized.
///
/// If the initialized plugin is not of type `P`.
pub(crate) fn with_plugin_state<P: 'static, R>(f: impl FnOnce(&P, PluginHandle<'_, P>) -> R) -> R {
    // usually this check would be looped to account for multiple reader threads trying to acquire it at the same time
    // but we expect there to be only one thread, so panic instead
    let state = STATE.load(Ordering::Relaxed);
    assert_ne!(state, LOCKED, "plugin invoked while (un)loading");
    assert_ne!(state + 1, LOCKED, "too many references to plugin state");
    STATE
        .compare_exchange(state, state + 1, Ordering::Relaxed, Ordering::Relaxed)
        .unwrap_or_else(|e| panic!("Plugin invoked concurrently (?), state: {}", e));
    defer! {{
        STATE
            .compare_exchange(state + 1, state, Ordering::Relaxed, Ordering::Relaxed)
            .unwrap_or_else(|e| panic!("Plugin invoked concurrently (?), state: {}", e));
    }}

    // Safety: STATE guarantees that there are only readers active
    let global_plugin = unsafe {
        (&*PLUGIN.get())
            .as_ref()
            .unwrap_or_else(|| panic!("Plugin invoked while uninitialized"))
    };

    #[cfg(debug_assertions)]
    debug_assert_eq!(
        global_plugin.thread_id,
        std::thread::current().id(),
        "plugin invoked from different thread"
    );

    let plugin = global_plugin
        .plugin
        .downcast_ref()
        .unwrap_or_else(|| panic!("Plugin is an unexpected type"));

    // Safety: we only store valid `plugin_handle`s in `PLUGIN`
    let raw = unsafe { RawPluginHandle::new(global_plugin.plugin_handle) };

    let ph = PluginHandle::new(raw);

    f(plugin, ph)
}
