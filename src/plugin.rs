use std::marker::PhantomData;

use crate::ffi::hexchat_plugin;

/// Must be implemented by all HexChat plugins.
///
/// Plugins must also implement `Default`, although it is not a superclass due to object safety restrictions.
pub trait HexchatPlugin: 'static {
    /// Initialize your plugin.
    ///
    /// Use this method to perform any work that should be done when your plugin is loaded,
    /// such as registering hooks or printing startup messages.
    ///
    /// Analogous to `hexchat_plugin_init`.
    fn init(&self, ph: PluginHandle<'_>);

    /// Deinitialize your plugin.
    ///
    /// Use this method to perform any work that should be done when your plugin is unloaded,
    /// such as printing shutdown messages or statistics.
    ///
    /// You do not need to explicitly `unhook` any hooks in this method, as remaining hooks are
    /// automatically removed by HexChat when your plugin finishes unloading.
    ///
    /// Analogous to `hexchat_plugin_deinit`.
    fn deinit(&self, ph: PluginHandle<'_>) {
        let _ = ph;
    }
}

/// The primary way to interact with HexChat's plugin API.
///
/// Cannot be constructed in user code, but is passed into `init`, `deinit`, and hook callbacks such as `hook_print`.
///
/// Analogous to `plugin_handle *ph`.
#[derive(Copy, Clone)]
pub struct PluginHandle<'ph> {
    /// Always points to a valid instance of `hexchat_plugin`.
    pub(crate) handle: *mut hexchat_plugin,
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

    /* TODO
        // general functions https://hexchat.readthedocs.io/en/latest/plugins.html#general-functions
        hexchat_command,
        hexchat_commandf,
        hexchat_print,
        hexchat_printf,
        hexchat_emit_print,
        hexchat_emit_print_attrs,
        hexchat_send_modes,
        hexchat_nickcmp,
        hexchat_strip,
        hexchat_free,
        hexchat_event_attrs_create,
        hexchat_event_attrs_free,
        // getting information https://hexchat.readthedocs.io/en/latest/plugins.html#getting-information
        hexchat_get_info,
        hexchat_get_prefs,
        hexchat_list_get,
        hexchat_list_fields,
        hexchat_list_next,
        hexchat_list_str,
        hexchat_list_int,
        hexchat_list_time,
        hexchat_list_free,
        // hook functions https://hexchat.readthedocs.io/en/latest/plugins.html#hook-functions
        hexchat_hook_command,
        hexchat_hook_fd,
        hexchat_hook_print,
        hexchat_hook_print_attrs,
        hexchat_hook_server,
        hexchat_hook_server_attrs,
        hexchat_hook_timer,
        hexchat_unhook,
        // context functions https://hexchat.readthedocs.io/en/latest/plugins.html#context-functions
        hexchat_find_context,
        hexchat_get_context,
        hexchat_set_context,
        // plugin preferences https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-preferences
        hexchat_pluginpref_set_str,
        hexchat_pluginpref_get_str,
        hexchat_pluginpref_set_int,
        hexchat_pluginpref_get_int,
        hexchat_pluginpref_delete,
        hexchat_pluginpref_list,
        // plugin gui https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-gui
        hexchat_plugingui_add,
        hexchat_plugingui_remove,
    */
}
