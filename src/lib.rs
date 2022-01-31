//! Write HexChat plugins in Rust.
//!
//! To create your plugin:
//! - Make a library crate with [`crate-type = "cdylib"`](https://doc.rust-lang.org/cargo/reference/manifest.html#building-dynamic-or-static-libraries).
//! - Define a type, e.g. `struct MyPlugin`, to hold any state your plugin needs.
//! - Implement the [`Plugin`] trait for `MyPlugin`.
//! - Call [`export_plugin`] with the type `MyPlugin`, its name, description, and version.
//!
//! On Windows, it is recommended to add `-C target-feature=+crt-static` to your `RUSTFLAGS`,
//! for example in [`<project root>/.cargo/config`](https://doc.rust-lang.org/cargo/reference/config.html).
//! This ensures that your DLL does not dynamically import the MSVCRT.
//!
//! # Examples
//!
//! The following is a port of HexChat's [example](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin) "auto-op" plugin.
//! It will automatically OP everyone who joins (so don't try this if you're in a real channel!),
//! and can be toggled on and off with `/autooptoggle`.
//!
//! ```rust
//! use std::cell::Cell;
//! use hexavalent::{Plugin, PluginHandle, export_plugin};
//! use hexavalent::event::print::Join;
//! use hexavalent::hook::{Eat, Priority};
//!
//! struct AutoOpPlugin {
//!     enabled: Cell<bool>,
//! }
//!
//! impl Default for AutoOpPlugin {
//!     fn default() -> Self {
//!         Self {
//!             enabled: Cell::new(true),
//!         }
//!     }
//! }
//!
//! impl AutoOpPlugin {
//!     fn autooptoggle_cb(&self, ph: PluginHandle<'_, Self>, _words: &[&str]) -> Eat {
//!         if !self.enabled.get() {
//!             self.enabled.set(true);
//!             ph.print("Auto-Oping now enabled!");
//!         } else {
//!             self.enabled.set(false);
//!             ph.print("Auto-Oping now disabled!");
//!         }
//!         // eat this command so HexChat and other plugins can't process it
//!         Eat::All
//!     }
//!
//!     fn join_cb(&self, ph: PluginHandle<'_, Self>, args: [&str; 4]) -> Eat {
//!         let [nick, _channel, _host, _account] = args;
//!         if self.enabled.get() {
//!             // op ANYONE who joins
//!             ph.command(&format!("OP {}", nick));
//!         }
//!         // don't eat this event, HexChat needs to see it
//!         Eat::None
//!     }
//! }
//!
//! impl Plugin for AutoOpPlugin {
//!     fn init(&self, ph: PluginHandle<'_, Self>) {
//!         ph.hook_command(
//!             "AutoOpToggle",
//!             "Usage: AUTOOPTOGGLE, turns OFF/ON Auto-Oping",
//!             Priority::Normal,
//!             Self::autooptoggle_cb,
//!         );
//!         ph.hook_print(Join, Priority::Normal, Self::join_cb);
//!
//!         ph.print("AutoOpPlugin loaded successfully!");
//!     }
//!
//!     fn deinit(&self, ph: PluginHandle<'_, Self>) {
//!         ph.print("Unloading AutoOpPlugin...");
//!     }
//! }
//!
//! export_plugin!(AutoOpPlugin, "AutoOp", "Auto-Ops anyone who joins", "0.1");
//! ```
//!
//! # Safety
//!
//! In general, this library depends on HexChat invoking the plugin from only one thread.
//! If that is not the case, this library provides no guarantees.
//! (Although it is never explicitly stated that this is true, HexChat's plugin documentation says nothing of synchronization,
//! and none of the example plugins have any. It also seems true in practice.)
//!
//! In debug mode (specifically, when `debug_assertions` is enabled), the current thread ID is checked every time the plugin is invoked,
//! which can help detect misbehavior.

#![allow(clippy::result_unit_err)]
#![warn(
    missing_debug_implementations,
    missing_docs,
    rustdoc::missing_doc_code_examples,
    trivial_numeric_casts,
    unreachable_pub,
    unused_qualifications
)]
#![deny(unsafe_op_in_unsafe_fn)]

#[macro_use]
mod macros;

mod ffi;
mod iter;
mod plugin;
mod state;

#[doc(hidden)]
pub mod internal;

pub mod context;
pub mod event;
pub mod gui;
pub mod hook;
pub mod info;
pub mod list;
pub mod mode;
pub mod pref;
pub mod strip;

pub use plugin::{Plugin, PluginHandle};

/// Defines the necessary exports for HexChat to load your plugin.
///
/// Do not define a `main` function; initialization should be performed in your plugin's [`Plugin::init`] function.
///
/// The type passed to `export_plugin` must implement [`Plugin`].
///
/// # Examples
///
/// ```rust
/// use hexavalent::{Plugin, PluginHandle, export_plugin};
///
/// #[derive(Default)]
/// struct NoopPlugin;
///
/// impl Plugin for NoopPlugin {
///     fn init(&self, ph: PluginHandle<'_, Self>) {
///         ph.print("Hello world!\0");
///     }
/// }
///
/// export_plugin!(NoopPlugin, "No-op", "Doesn't do anything", "1.0.0");
/// ```
///
/// Cargo's environment variables can also be used to copy `name`, `description`, and `version` from `Cargo.toml`.
///
/// ```rust
/// use hexavalent::{Plugin, PluginHandle, export_plugin};
///
/// #[derive(Default)]
/// struct NoopPlugin;
///
/// impl Plugin for NoopPlugin {
///     fn init(&self, ph: PluginHandle<'_, Self>) {
///         ph.print("Hello world!\0");
///     }
/// }
///
/// export_plugin!(
///     NoopPlugin,
///     env!("CARGO_PKG_NAME"),
///     env!("CARGO_PKG_DESCRIPTION"),
///     env!("CARGO_PKG_VERSION"),
/// );
/// ```
#[macro_export]
macro_rules! export_plugin {
    (
        $plugin_ty:ty,
        $name:expr,
        $desc:expr,
        $version:expr $(,)?
    ) => {
        #[no_mangle]
        pub unsafe extern "C" fn hexchat_plugin_init(
            plugin_handle: *mut $crate::internal::hexchat_plugin,
            plugin_name: *mut *const ::std::os::raw::c_char,
            plugin_desc: *mut *const ::std::os::raw::c_char,
            plugin_version: *mut *const ::std::os::raw::c_char,
            _arg: *mut ::std::os::raw::c_char,
        ) -> ::std::os::raw::c_int {
            // Safety: these literals are null-terminated and 'static
            const NAME: &'static str = concat!($name, "\0");
            const DESC: &'static str = concat!($desc, "\0");
            const VERSION: &'static str = concat!($version, "\0");
            // note that these user-provided strings may contain interior nulls, so we cannot go through &CStr
            // it's fine to go straight to `*const c_char` though, as C doesn't care about that, it'll just end the string early
            *plugin_name = NAME.as_ptr().cast();
            *plugin_desc = DESC.as_ptr().cast();
            *plugin_version = VERSION.as_ptr().cast();

            $crate::internal::hexchat_plugin_init::<$plugin_ty>(plugin_handle)
        }

        #[no_mangle]
        pub unsafe extern "C" fn hexchat_plugin_deinit(
            plugin_handle: *mut $crate::internal::hexchat_plugin,
        ) -> ::std::os::raw::c_int {
            $crate::internal::hexchat_plugin_deinit::<$plugin_ty>(plugin_handle)
        }
    };
}
