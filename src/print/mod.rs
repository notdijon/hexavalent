//! Types related to print events.
//!
//! A list of all print events can be viewed in HexChat under Settings > Text Events.
//!
//! # Examples
//!
//! ```rust
//! use hexavalent::PluginHandle;
//! use hexavalent::print::ChannelMessage;
//!
//! fn print_welcome_message(ph: PluginHandle<'_>) -> Result<(), ()> {
//!     ph.emit_print(ChannelMessage, ["hexavalent\0", "Plugin started!\0", "@\0", "\0"])
//! }
//! ``````

use std::ffi::CStr;
use std::os::raw::c_char;

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
    /// If `c_args` is too small or contains invalid data.
    #[doc(hidden)]
    fn c_to_args(c_args: &[&'a CStr]) -> Self::Args;
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
        #[doc = "Fields: "]
        #[doc = "["]
        $(
            #[doc = "`"]
            #[doc = $field_name]
            #[doc = "`, "]
        )*
        #[doc = "]."]
        #[doc = ""]
        #[doc = "Format: `"]
        #[doc = $event_format]
        #[doc = "`."]
        pub struct $struct_name;

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
            type Args = [&'a str; count!($($index)*)];

            #[doc(hidden)]
            #[allow(unused_variables)]
            fn args_to_c<R>(args: Self::Args, f: impl FnOnce(&[&::std::ffi::CStr]) -> R) -> R {
                let args: [::std::borrow::Cow::<'_, ::std::ffi::CStr>; count!($($index)*)] = [
                    $(crate::ffi::StrExt::into_cstr(args[$index])),*
                ];
                let args = [
                    $(args[$index].as_ref()),*
                ];
                f(&args)
            }

            #[doc(hidden)]
            fn c_to_args(c_args: &[&'a ::std::ffi::CStr]) -> Self::Args {
                assert_eq!(c_args.len(), count!($($index)*), "Incorrect number of fields in event '{}'", $event_name);
                [
                    $(
                        c_args[$index].to_str().unwrap_or_else(|e| {
                            panic!(
                                "Error parsing index {} of event '{}': {}",
                                stringify!($index),
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

mod impls;

pub use impls::*;
