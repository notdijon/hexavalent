//! This module exists solely to provide exports for the `hexchat_plugin` macro to use.
//!
//! DO NOT IMPORT OR USE ANYTHING FROM THIS MODULE

/// UNSTABLE: do not use this type directly.
///
/// Used by the `hexchat_plugin` macro.
#[doc(hidden)]
pub use crate::ffi::hexchat_plugin;

/// UNSTABLE: do not call this function directly.
///
/// Used by the `hexchat_plugin` macro.
#[doc(hidden)]
pub use crate::state::hexchat_plugin_init;

/// UNSTABLE: do not call this function directly.
///
/// Used by the `hexchat_plugin` macro.
#[doc(hidden)]
pub use crate::state::hexchat_plugin_deinit;
