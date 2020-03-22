//! This module exists solely to provide exports for the `hexchat_plugin` macro to use.
//!
//! DO NOT IMPORT OR USE ANYTHING FROM THIS MODULE

use std::os::raw::c_int;

use crate::plugin::Plugin;
use crate::state;

/// UNSTABLE: do not use this type.
///
/// Used by the `hexchat_plugin` macro.
#[doc(hidden)]
pub use crate::ffi::hexchat_plugin;

/// UNSTABLE: do not call this function.
///
/// Used by the `hexchat_plugin` macro.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
#[doc(hidden)]
pub unsafe fn hexchat_plugin_init<P: Plugin>(plugin_handle: *mut hexchat_plugin) -> c_int {
    state::hexchat_plugin_init::<P>(plugin_handle)
}

/// UNSTABLE: do not call this function.
///
/// Used by the `hexchat_plugin` macro.
///
/// # Safety
///
/// `plugin_handle` must point to a valid `hexchat_plugin`.
#[doc(hidden)]
pub unsafe fn hexchat_plugin_deinit<P: Plugin>(plugin_handle: *mut hexchat_plugin) -> c_int {
    state::hexchat_plugin_deinit::<P>(plugin_handle)
}
