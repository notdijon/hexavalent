use std::marker::PhantomData;
use std::os::raw::c_char;
use std::ptr;

use crate::ffi::{hexchat_plugin, int_to_result, StrExt};
use crate::print::PrintEvent;

/// Must be implemented by all HexChat plugins.
///
/// Plugins must also implement `Default`, although it is not a superclass due to object safety restrictions.
///
/// # Examples
///
/// TODO add example when more stuff works
///  printing statistics would be good here
pub trait HexchatPlugin: 'static {
    /// Initialize your plugin.
    ///
    /// Use this method to perform any work that should be done when your plugin is loaded,
    /// such as registering hooks or printing startup messages.
    ///
    /// Analogous to [`hexchat_plugin_init`](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::{HexchatPlugin, PluginHandle};
    ///
    /// struct MyPlugin;
    ///
    /// impl HexchatPlugin for MyPlugin {
    ///     fn init(&self, ph: PluginHandle<'_>) {
    ///         ph.print("Plugin loaded successfully!\0");
    ///     }
    /// }
    /// ```
    fn init(&self, ph: PluginHandle<'_>);

    /// Deinitialize your plugin.
    ///
    /// Use this method to perform any work that should be done when your plugin is unloaded,
    /// such as printing shutdown messages or statistics.
    ///
    /// You do not need to explicitly `unhook` any hooks in this method, as remaining hooks are
    /// automatically removed by HexChat when your plugin finishes unloading.
    ///
    /// Analogous to [`hexchat_plugin_deinit`](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::{HexchatPlugin, PluginHandle};
    ///
    /// struct MyPlugin;
    ///
    /// impl HexchatPlugin for MyPlugin {
    ///     fn init(&self, _: PluginHandle<'_>) {}
    ///
    ///     fn deinit(&self, ph: PluginHandle<'_>) {
    ///         ph.print("Plugin unloading...\0");
    ///     }
    /// }
    /// ```
    fn deinit(&self, ph: PluginHandle<'_>) {
        let _ = ph;
    }
}

/// Interacts with HexChat's plugin API.
///
/// Cannot be constructed in user code, but is passed into [`init`](trait.HexchatPlugin.html#tymethod.init), [`deinit`](trait.HexchatPlugin.html#method.deinit),
/// and hook callbacks such as [`hook_print`](struct.PluginHandle.html#method.hook_print).
///
/// Most of HexChat's [functions](https://hexchat.readthedocs.io/en/latest/plugins.html#functions) are available as struct methods,
/// without the `hexchat_` prefix.
///
/// # Examples
///
/// All functions which take `&str` arguments will allocate if the string is not null-terminated, and panic if the string contains interior nulls.
///
/// ```rust
/// # fn print_some_stuff(ph: hexavalent::PluginHandle<'_>) {
/// // for example, this would not allocate
/// ph.print("hello\0");
/// // ...this would allocate
/// ph.print("hello");
/// // ...and this would panic
/// ph.print("hel\0lo");
/// # }
/// ```
///
/// TODO add basic hook example
#[derive(Copy, Clone)]
pub struct PluginHandle<'ph> {
    /// Always points to a valid instance of `hexchat_plugin`.
    handle: *mut hexchat_plugin,
    _lifetime: PhantomData<&'ph hexchat_plugin>,
}

impl<'ph> PluginHandle<'ph> {
    /// Creates a new `PluginHandle` from a native `hexchat_plugin`.
    ///
    /// # Safety
    ///
    /// `plugin_handle` must point to a valid instance of `hexchat_plugin`.
    pub(crate) unsafe fn new(plugin_handle: *mut hexchat_plugin) -> Self {
        Self {
            handle: plugin_handle,
            _lifetime: PhantomData,
        }
    }
}

/// [General Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#general-functions)
impl<'ph> PluginHandle<'ph> {
    /// Prints text to the current tab. Text may contain mIRC color codes.
    ///
    /// Analogous to [`hexchat_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_print).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn say_hello(ph: PluginHandle<'_>) {
    ///     // null-termination is not required, but avoids allocation
    ///     ph.print("hello!\0");
    /// }
    /// ```
    pub fn print(self, text: &str) {
        text.with_cstr(|text| {
            // Safety: `handle` is always valid
            unsafe {
                ((*self.handle).hexchat_print)(self.handle, text.as_ptr());
            }
        });
    }

    /// Executes a command as if it were typed into HexChat's input box after a `/`.
    ///
    /// Analogous to [`hexchat_command`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_command).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn op_user(ph: PluginHandle<'_>, username: &str) {
    ///     // do not include the leading slash
    ///     ph.command(&format!("OP {}\0", username));
    /// }
    /// ```
    pub fn command(self, cmd: &str) {
        cmd.with_cstr(|cmd| {
            // Safety: `handle` is always valid
            unsafe {
                ((*self.handle).hexchat_command)(self.handle, cmd.as_ptr());
            }
        })
    }

    /// Emits a print event.
    ///
    /// Returns whether emission succeeded or failed.
    ///
    /// A list of print events can be found in HexChat, under Settings > Text Events.
    /// You can also look at the implementations of [`PrintEvent`](print/trait.PrintEvent.html).
    ///
    /// If you do not know the print event's type statically, use [`emit_print_dyn`](struct.PluginHandle.html#method.emit_print_dyn).
    ///
    /// Analogous to [`hexchat_emit_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    /// use hexavalent::print::events::ChannelMessage;
    ///
    /// fn print_fake_message(ph: PluginHandle<'_>, user: &str, text: &str) -> Result<(), ()> {
    ///     ph.emit_print(ChannelMessage, [user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print<'a, E: PrintEvent<'a>>(
        self,
        event: E,
        args: <E as PrintEvent<'a>>::Args,
    ) -> Result<(), ()> {
        let _ = event;
        E::args_to_c(args, |args| {
            assert!(
                args.len() <= 4,
                "bug in hexavalent - more than 4 args from PrintEvent"
            );

            let args: [*const c_char; 4] = [
                args.get(0).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(1).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(2).map_or_else(ptr::null, |a| a.as_ptr()),
                args.get(3).map_or_else(ptr::null, |a| a.as_ptr()),
            ];

            // Safety: `handle` is always valid
            int_to_result(unsafe {
                ((*self.handle).hexchat_emit_print)(
                    self.handle,
                    E::NAME,
                    args[0],
                    args[1],
                    args[2],
                    args[3],
                    ptr::null::<c_char>(),
                )
            })
        })
    }

    /// Emits a print event, with dynamic type.
    ///
    /// Returns whether emission succeeded or failed.
    ///
    /// A list of print events can be found in HexChat, under Settings > Text Events.
    /// You can also look at the implementations of [`PrintEvent`](print/trait.PrintEvent.html).
    ///
    /// Prefer [`emit_print`](struct.PluginHandle.html#method.emit_print) if you know the print event's type statically.
    ///
    /// Analogous to [`hexchat_emit_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print).
    ///
    /// # Panics
    ///
    /// If `args` contains more than 4 elements. (No text event takes more than 4 arguments.)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use hexavalent::PluginHandle;
    ///
    /// fn print_fake_message(ph: PluginHandle<'_>, user: &str, text: &str) -> Result<(), ()> {
    ///     ph.emit_print_dyn("Channel Message\0", &[user, text, "@\0", "$\0"])
    /// }
    /// ```
    pub fn emit_print_dyn(self, event: &str, args: &[&str]) -> Result<(), ()> {
        assert!(
            args.len() <= 4,
            "passed {} args to text event {}, but no text event takes more than 4 args",
            args.len(),
            event
        );
        event.with_cstr(|event| {
            let args = [
                args.get(0).map(|&s| s.into_cstr()),
                args.get(1).map(|&s| s.into_cstr()),
                args.get(2).map(|&s| s.into_cstr()),
                args.get(3).map(|&s| s.into_cstr()),
            ];
            let args: [*const c_char; 4] = [
                args[0].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
                args[1].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
                args[2].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
                args[3].as_ref().map_or_else(ptr::null, |a| a.as_ptr()),
            ];

            // Safety: `handle` is always valid
            int_to_result(unsafe {
                ((*self.handle).hexchat_emit_print)(
                    self.handle,
                    event.as_ptr(),
                    args[0],
                    args[1],
                    args[2],
                    args[3],
                    ptr::null::<c_char>(),
                )
            })
        })
    }
    /* TODO
        hexchat_emit_print,
        hexchat_emit_print_attrs,
        hexchat_send_modes,
        hexchat_nickcmp,
        hexchat_strip,
        hexchat_free,
        hexchat_event_attrs_create,
        hexchat_event_attrs_free,
    */
}

/// [Getting Information](https://hexchat.readthedocs.io/en/latest/plugins.html#getting-information)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_get_info,
        hexchat_get_prefs,
        hexchat_list_get,
        hexchat_list_fields,
        hexchat_list_next,
        hexchat_list_str,
        hexchat_list_int,
        hexchat_list_time,
        hexchat_list_free,
    */
}

/// [Hook Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#hook-functions)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_hook_command,
        hexchat_hook_fd,
        hexchat_hook_print,
        hexchat_hook_print_attrs,
        hexchat_hook_server,
        hexchat_hook_server_attrs,
        hexchat_hook_timer,
        hexchat_unhook,
    */
}

/// [Context Functions](https://hexchat.readthedocs.io/en/latest/plugins.html#context-functions)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_find_context,
        hexchat_get_context,
        hexchat_set_context,
    */
}

/// [Plugin Preferences](https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-preferences)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_pluginpref_set_str,
        hexchat_pluginpref_get_str,
        hexchat_pluginpref_set_int,
        hexchat_pluginpref_get_int,
        hexchat_pluginpref_delete,
        hexchat_pluginpref_list,
    */
}

/// [Plugin GUI](https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-gui)
impl<'ph> PluginHandle<'ph> {
    /* TODO
        hexchat_plugingui_add,
        hexchat_plugingui_remove,
    */
}
