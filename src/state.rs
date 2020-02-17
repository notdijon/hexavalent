use std::any::Any;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::os::raw::c_int;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread::{self, ThreadId};
use std::usize;

use crate::ffi::{catch_and_log_unwind, hexchat_plugin, result_to_int};
use crate::plugin::{Plugin, PluginHandle};

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

/// Global handle to the user's plugin data and the global HexCHat plugin context.
///
/// Only accessible outside this module via the safe interface `with_plugin_state`.
static PLUGIN: ExtSync<Option<(ThreadId, Box<dyn Any>, *mut hexchat_plugin)>> =
    ExtSync(UnsafeCell::new(None));

/// Initializes a plugin of type `P`.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
///
/// # Panics
///
/// If the plugin is running and currently holds a reference to the plugin state.
pub unsafe fn hexchat_plugin_init<P: Plugin + Default>(
    plugin_handle: *mut hexchat_plugin,
) -> c_int {
    // Safety: `plugin_handle` points to a valid `hexchat_plugin`
    let ph = PluginHandle::new(plugin_handle);
    result_to_int(catch_and_log_unwind(ph, "init", || {
        {
            let replaced_state = STATE.compare_and_swap(NO_READERS, LOCKED, Ordering::SeqCst);
            assert_eq!(replaced_state, NO_READERS, "initialized while running");
            defer! { STATE.store(NO_READERS, Ordering::SeqCst) };

            // Safety: STATE guarantees unique access to handles
            *PLUGIN.get() = Some((
                thread::current().id(),
                Box::new(P::default()),
                plugin_handle,
            ));
        }

        with_plugin_state(|this: &P, ph| this.init(ph));
    }))
}

/// Deinitializes a plugin of type `P`.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
///
/// # Panics
///
/// If the plugin is running and currently holds a reference to the plugin state.
pub unsafe fn hexchat_plugin_deinit<P: Plugin>(plugin_handle: *mut hexchat_plugin) -> c_int {
    // Safety: `plugin_handle` points to a valid `hexchat_plugin`
    let ph = PluginHandle::new(plugin_handle);
    result_to_int(catch_and_log_unwind(ph, "deinit", || {
        with_plugin_state(|this: &P, ph| this.deinit(ph));

        {
            let replaced_state = STATE.compare_and_swap(NO_READERS, LOCKED, Ordering::SeqCst);
            assert_eq!(replaced_state, NO_READERS, "deinitialized while running");
            defer! { STATE.store(NO_READERS, Ordering::SeqCst) };

            // Safety: LOCK guarantees unique access to handles
            *PLUGIN.get() = None;
        }
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
pub fn with_plugin_state<P: Plugin, R>(f: impl FnOnce(&P, PluginHandle<'_>) -> R) -> R {
    // usually this check would be looped to account for multiple reader threads trying to acquire it at the same time
    // but we expect there to be only one thread, so panic instead
    let old_state = STATE.load(Ordering::Relaxed);
    assert_ne!(old_state, LOCKED, "plugin invoked while (un)loading");
    let replaced_state = STATE.compare_and_swap(old_state, old_state + 1, Ordering::SeqCst);
    assert_ne!(replaced_state, LOCKED, "plugin invoked while (un)loading");
    assert_eq!(replaced_state, old_state, "plugin invoked concurrently (?)");
    defer! { STATE.fetch_sub(1, Ordering::SeqCst) };

    // Safety: STATE guarantees that there are only readers active
    let (thread_id, user_handle, plugin_handle) = unsafe {
        (&*PLUGIN.get())
            .as_ref()
            .unwrap_or_else(|| panic!("plugin invoked while uninitialized"))
    };

    debug_assert_eq!(*thread_id, thread::current().id());

    let user_handle = user_handle
        .downcast_ref()
        .unwrap_or_else(|| panic!("stored plugin is an unexpected type"));

    // Safety: we only store valid `plugin_handle`s in `PLUGIN`
    let plugin_handle = unsafe { PluginHandle::new(*plugin_handle) };

    f(user_handle, plugin_handle)
}
