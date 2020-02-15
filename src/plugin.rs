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
}
