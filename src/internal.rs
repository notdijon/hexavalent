//! This module exists solely to provide exports for the `hexchat_plugin` macro to use.
//!
//! DO NOT IMPORT OR USE ANYTHING FROM THIS MODULE

use std::os::raw::c_int;

use crate::plugin::HexchatPlugin;
use crate::state;

/// Internal type used by the `hexchat_plugin` macro.
/// Do not use this type directly.
#[doc(hidden)]
pub use crate::ffi::hexchat_plugin;

/// Internal function used by the `hexchat_plugin` macro.
/// Do not call this function directly.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
#[doc(hidden)]
pub unsafe fn hexchat_plugin_init<P: HexchatPlugin + Default>(
    plugin_handle: *mut hexchat_plugin,
) -> c_int {
    state::hexchat_plugin_init::<P>(plugin_handle)
}

/// Internal function used by the `hexchat_plugin` macro.
/// Do not call this function directly.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
#[doc(hidden)]
pub unsafe fn hexchat_plugin_deinit<P: HexchatPlugin>(plugin_handle: *mut hexchat_plugin) -> c_int {
    state::hexchat_plugin_deinit::<P>(plugin_handle)
}
