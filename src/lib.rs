//! Write Hexchat plugins in Rust.
//!
//! To create your plugin, make a library crate with `crate-type = "cdylib"`.
//! Define a type to hold any state your plugin will need.
//! Implement the [`HexchatPlugin`](trait.HexchatPlugin.html) trait for plugin (de)initialization,
//! and the `Default` trait so this library can create an instance of your plugin.
//! Finally,  call [`export_plugin`](macro.export_plugin.html) with the type of your plugin, its name, description, and version.
//!
//! On Windows, it is recommended to add `-C target-feature=+crt-static` to your `RUSTFLAGS`,
//! for example in [`<project root>/.cargo/config`](https://doc.rust-lang.org/cargo/reference/config.html).
//! This will ensure that your DLL does not dynamically import the MSVCRT.
//!
//! # Examples
//!
//! The following is HexChat's [example](https://hexchat.readthedocs.io/en/latest/plugins.html#sample-plugin) "auto-op" plugin.
//! TODO add example when more stuff works
//!
//! # Safety
//!
//! In general, this library depends on HexChat invoking the plugin from only one thread.
//! If that is not the case, this library provides no guarantees.
//! (Although it is never explicitly stated that this is true, HexChat's plugin documentation says nothing of synchronization,
//! and none of the example plugins have any. It also seems true in practice.)
//!
//! In debug node (specifically, when `debug_assertions` is enabled), the current thread ID will be checked every time the plugin is invoked,
//! which can help detect misbehavior.

#![warn(missing_docs)]

#[macro_use]
mod macros;

mod ffi;
mod plugin;
mod state;

#[doc(hidden)]
pub mod internal;

pub use plugin::{HexchatPlugin, PluginHandle};

/// Defines the necessary exports for HexChat to load your plugin.
///
/// Do not define a `main` function; initialization should be performed in your plugin's [`init`](trait.HexchatPlugin.html#tymethod.init) function.
///
/// The type passed to `export_plugin` must implement both [`HexchatPlugin`](trait.HexchatPlugin.html) and `Default`.
///
/// # Examples
///
/// ```rust
/// # use hexavalent::{HexchatPlugin, PluginHandle, export_plugin};
/// #[derive(Default)]
/// struct NoopPlugin;
///
/// impl HexchatPlugin for NoopPlugin {
///     fn init(&self, ph: PluginHandle<'_>) {
///         ph.print("Hello world!\0");
///     }
/// }
///
/// export_plugin!(NoopPlugin, name: "No-op Plugin", desc: "Does nothing.", version: "1.0.0");
/// ```
///
/// Cargo's environment variables can also be used to copy `name`, `description`, and `version` from `Cargo.toml`.
///
/// ```rust
/// # use hexavalent::{HexchatPlugin, PluginHandle, export_plugin};
/// # #[derive(Default)]
/// # struct NoopPlugin;
/// # impl HexchatPlugin for NoopPlugin {
/// #     fn init(&self, ph: PluginHandle<'_>) {}
/// # }
/// export_plugin!(
///     NoopPlugin,
///     name: env!("CARGO_PKG_NAME"),
///     desc: env!("CARGO_PKG_DESCRIPTION"),
///     version: env!("CARGO_PKG_VERSION"),
/// );
/// ```
#[macro_export]
macro_rules! export_plugin {
    (
        $plugin_ty:ty,
        name: $name:expr,
        desc: $desc:expr,
        version: $version:expr $(,)?
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
