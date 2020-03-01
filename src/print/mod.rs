//! Types related to print events.
//!
//! # Examples
//!
//! Emitting a print event.
//!
//! ```rust
//! use hexavalent::PluginHandle;
//! use hexavalent::print::events::ChannelMessage;
//!
//! fn print_welcome_message<P>(ph: PluginHandle<'_, P>) -> Result<(), ()> {
//!     ph.emit_print(ChannelMessage, ["hexavalent\0", "Plugin started!\0", "@\0", "\0"])
//! }
//! ```
//!
//! Registering a hook for a print event.
//!
//! ```rust
//! use hexavalent::PluginHandle;
//! use hexavalent::hook::{Eat, Priority};
//! use hexavalent::print::PrintEvent;
//! use hexavalent::print::events::ChannelMessage;
//!
//! fn hook_message<P: 'static>(ph: PluginHandle<'_, P>) {
//!     ph.hook_print(ChannelMessage, Priority::Normal, message_cb::<P>);
//! }
//!
//! fn message_cb<P>(
//!     plugin: &P,
//!     ph: PluginHandle<'_, P>,
//!     args: <ChannelMessage as PrintEvent<'_>>::Args,
//! ) -> Eat {
//!     let [nick, text, mode, ident] = args;
//!     ph.print(&format!(
//!         "Message from {} (with mode '{}', ident '{}'): {}\0",
//!         nick, mode, ident, text
//!     ));
//!     Eat::HexChat
//! }
//! ```

use std::ffi::CStr;
use std::marker::PhantomData;
use std::os::raw::c_char;

use time::OffsetDateTime;

/// Attributes associated with a print event.
///
/// Used with [`PluginHandle::emit_print_attrs`](../struct.PluginHandle.html#method.emit_print_attrs)
/// and [`PluginHandle::hook_print_attrs`](../struct.PluginHandle.html#method.hook_print_attrs).
///
/// Analogous to [`hexchat_event_attrs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print_attrs).
#[derive(Copy, Clone)]
pub struct EventAttrs<'a> {
    time: OffsetDateTime,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> EventAttrs<'a> {
    /// Creates a new `EventAttrs` from the specified event timestamp.
    pub fn new(time: OffsetDateTime) -> Self {
        Self {
            time,
            _lifetime: PhantomData,
        }
    }

    /// Gets the timestamp associated with this event.
    pub fn time(self) -> OffsetDateTime {
        self.time
    }
}

/// Trait implemented by all print event types.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub unsafe trait PrintEvent<'a>: private::PrintEventImpl {
    /// The event's name.
    ///
    /// # Safety
    ///
    /// Must point to a valid, null-terminated C-style string.
    const NAME: *const c_char;

    /// The arguments associated with this event.
    type Args: AsRef<[&'a str]>;

    /// UNSTABLE: do not call this function directly.
    ///
    /// Converts this event's args to C-style strings.
    #[doc(hidden)]
    fn args_to_c<R>(args: Self::Args, f: impl FnOnce(&[&CStr]) -> R) -> R;

    /// UNSTABLE: do not call this function directly.
    ///
    /// Converts an array of C-style strings to this event's args.
    ///
    /// # Panics
    ///
    /// If `c_args` contains invalid UTF8.
    #[doc(hidden)]
    fn c_to_args(c_args: impl Iterator<Item = &'a CStr>) -> Self::Args;
}

pub(crate) mod private {
    pub trait PrintEventImpl {}
}

macro_rules! count {
    () => {
        0
    };
    ($x:tt $($xs:tt)*) => {
        1 + count!($($xs)*)
    };
}

macro_rules! print_event {
    ($struct_name:ident, $event_name:literal, $event_format:literal, $($index:literal : $field_name:literal),*) => {
        #[doc = "`"]
        #[doc = $event_name]
        #[doc = "`"]
        #[doc = ""]
        #[doc = "`"]
        #[doc = $event_format]
        #[doc = "`"]
        #[doc = ""]
        #[doc = "Fields: "]
        #[doc = "["]
        $(
            #[doc = "`"]
            #[doc = $field_name]
            #[doc = "`, "]
        )*
        #[doc = "]."]
        pub struct $struct_name;

        impl $struct_name {
            const FIELD_COUNT: usize = count!($($index)*);
        }

        impl crate::print::private::PrintEventImpl for $struct_name {}

        unsafe impl<'a> crate::print::PrintEvent<'a> for $struct_name {
            #[doc = "`"]
            #[doc = $event_name]
            #[doc = "`"]
            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($event_name, "\0").as_ptr().cast();

            #[doc = "["]
            $(
                #[doc = ""]
                #[doc = "`"]
                #[doc = $field_name]
                #[doc = "`, "]
            )*
            #[doc = ""]
            #[doc = "]"]
            type Args = [&'a str; Self::FIELD_COUNT];

            #[doc(hidden)]
            #[allow(unused_variables)]
            fn args_to_c<R>(args: Self::Args, f: impl FnOnce(&[&::std::ffi::CStr]) -> R) -> R {
                let args: [::std::borrow::Cow::<'_, ::std::ffi::CStr>; Self::FIELD_COUNT] = [
                    $(crate::ffi::StrExt::into_cstr(args[$index])),*
                ];
                let args = [
                    $(args[$index].as_ref()),*
                ];
                f(&args)
            }

            #[doc(hidden)]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn c_to_args(mut c_args: impl Iterator<Item = &'a ::std::ffi::CStr>) -> Self::Args {
                [
                    $(
                        c_args
                            .next()
                            .unwrap_or_else(|| {
                                panic!(
                                    "Insufficient fields in event '{}': expected {}, found {}",
                                     $event_name,
                                     Self::FIELD_COUNT,
                                     $index
                                 )
                            })
                            .to_str()
                            .unwrap_or_else(|e| {
                                panic!(
                                    "Invalid UTF8 in field index {} of event '{}': {}",
                                    $index,
                                    $event_name,
                                    e,
                                )
                            })
                    ),*
                ]
            }
        }
    };
}

/// Print event types.
///
/// A list of all print events can also be viewed in HexChat under Settings > Text Events.
pub mod events;

/// Special print event types which can only be hooked, not emitted.
///
/// Used with hook registration functions such as [`PluginHandle::hook_print`](../../struct.PluginHandle.html#method.hook_print).
///
/// Attempting to emit these events with emission functions such as [`PluginHandle::emit_print`](../../struct.PluginHandle.html#method.emit_print) will always fail.
///
/// Analogous to the special print events documented for [`hexchat_hook_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_print).
pub mod special;
