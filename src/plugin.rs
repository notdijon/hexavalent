use std::marker::PhantomData;

use crate::cstr::IntoCstr;
use crate::ffi::hexchat_plugin;

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
    /// # use hexavalent::{HexchatPlugin, PluginHandle};
    /// # struct MyPlugin;
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
    /// # use hexavalent::{HexchatPlugin, PluginHandle};
    /// # struct MyPlugin;
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
    /// # use hexavalent::PluginHandle;
    /// fn say_hello(ph: PluginHandle<'_>) {
    ///     // null-termination is not required, but avoids allocation
    ///     ph.print("hello!\0");
    /// }
    /// ```
    pub fn print(self, text: impl IntoCstr) {
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
    /// # use hexavalent::PluginHandle;
    /// fn op_user(ph: PluginHandle<'_>, username: &str) {
    ///     // do not include the leading slash
    ///     ph.command(format!("OP {}", username));
    /// }
    /// ```
    pub fn command(self, cmd: impl IntoCstr) {
        cmd.with_cstr(|cmd| {
            // Safety: `handle` is always valid
            unsafe {
                ((*self.handle).hexchat_command)(self.handle, cmd.as_ptr());
            }
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
