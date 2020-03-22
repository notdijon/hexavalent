//! Print and server events.

use std::ffi::CStr;
use std::marker::PhantomData;

use time::OffsetDateTime;

/// Attributes associated with an event.
///
/// Used with [`PluginHandle::emit_print_attrs`](../struct.PluginHandle.html#method.emit_print_attrs),
/// [`PluginHandle::hook_print_attrs`](../struct.PluginHandle.html#method.hook_print_attrs),
/// and [`PluginHandle::hook_server_attrs`](../struct.PluginHandle.html#method.hook_server_attrs).
///
/// Analogous to [`hexchat_event_attrs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print_attrs).
#[derive(Debug, Copy, Clone)]
pub struct EventAttrs<'a> {
    time: OffsetDateTime,
    #[cfg(feature = "__unstable_ircv3_line_in_event_attrs")]
    ircv3_line: &'a str,
    _lifetime: PhantomData<&'a ()>,
}

impl<'a> EventAttrs<'a> {
    /// Creates a new `EventAttrs` from the specified event timestamp.
    pub fn new(
        time: OffsetDateTime,
        #[cfg(feature = "__unstable_ircv3_line_in_event_attrs")] ircv3_line: &'a str,
    ) -> Self {
        Self {
            time,
            #[cfg(feature = "__unstable_ircv3_line_in_event_attrs")]
            ircv3_line,
            _lifetime: PhantomData,
        }
    }

    /// Gets the timestamp associated with this event.
    pub fn time(self) -> OffsetDateTime {
        self.time
    }

    /// Gets the IRCv3 line associated with this event.
    #[cfg(feature = "__unstable_ircv3_line_in_event_attrs")]
    pub fn ircv3_line(self) -> &'a str {
        self.ircv3_line
    }
}

/// Trait implemented by all event types.
///
/// See the [`PrintEvent`](print/trait.PrintEvent.html) and [`ServerEvent`](server/trait.ServerEvent.html) traits for usage.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Event<'a>: private::EventImpl {
    /// The arguments associated with this event.
    type Args: AsRef<[&'a str]>;

    /// UNSTABLE: do not call this function.
    ///
    /// Converts this event's args to C-style strings.
    #[doc(hidden)]
    fn args_to_c<R>(args: Self::Args, f: impl FnOnce(&[&CStr]) -> R) -> R;

    /// UNSTABLE: do not call this function.
    ///
    /// Converts an array of C-style strings to this event's args.
    ///
    /// # Panics
    ///
    /// If `word` or `word_eol` contains invalid UTF8.
    #[doc(hidden)]
    fn args_from_words(
        word: impl Iterator<Item = &'a ::std::ffi::CStr>,
        word_eol: impl Iterator<Item = &'a ::std::ffi::CStr>,
    ) -> Self::Args;
}

pub(crate) mod private {
    use std::os::raw::c_char;

    pub unsafe trait EventImpl {
        /// The event's name.
        ///
        /// # Safety
        ///
        /// Must point to a valid, null-terminated C-style string.
        const NAME: *const c_char;
    }
}

macro_rules! count {
    () => {
        0
    };
    ($x:tt $($xs:tt)*) => {
        1 + count!($($xs)*)
    };
}

macro_rules! event {
    (
        $struct_name:ident,
        $event_name:literal,
        $event_doc:literal,
        $($index:literal : $field_name:literal),*
        $(, eol $eol_index:literal : $eol_name:literal)?
    ) => {
        #[doc = "`"]
        #[doc = $event_name]
        #[doc = "`"]
        #[doc = ""]
        #[doc = $event_doc]
        #[doc = ""]
        #[doc = "Fields: "]
        #[doc = "["]
        $(
            #[doc = "`"]
            #[doc = $field_name]
            #[doc = "`, "]
        )*
        $(
            #[doc = "`"]
            #[doc = $eol_name]
            #[doc = "`, "]
        )?
        #[doc = "]."]
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name;

        impl $struct_name {
            const FIELD_COUNT: usize = count!($($index)* $($eol_index)?);
        }

        unsafe impl crate::event::private::EventImpl for $struct_name {
            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($event_name, "\0").as_ptr().cast();
        }

        impl<'a> crate::event::Event<'a> for $struct_name {
            $(
                #[doc = ""]
                #[doc = "`"]
                #[doc = $field_name]
                #[doc = "`, "]
            )*
            $(
                #[doc = ""]
                #[doc = "`"]
                #[doc = $eol_name]
                #[doc = "`, "]
            )?
            type Args = [&'a str; Self::FIELD_COUNT];

            #[doc(hidden)]
            #[allow(unused_variables)]
            fn args_to_c<R>(args: Self::Args, f: impl FnOnce(&[&::std::ffi::CStr]) -> R) -> R {
                let args: [::std::borrow::Cow::<'_, ::std::ffi::CStr>; Self::FIELD_COUNT] = [
                    $(crate::ffi::StrExt::into_cstr(args[$index])),*
                    $(, crate::ffi::StrExt::into_cstr(args[$eol_index]))?
                ];
                let args = [
                    $(args[$index].as_ref()),*
                    $(, args[$eol_index].as_ref())?
                ];
                f(&args)
            }

            #[doc(hidden)]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn args_from_words(
                mut word: impl Iterator<Item = &'a ::std::ffi::CStr>,
                mut word_eol: impl Iterator<Item = &'a ::std::ffi::CStr>,
            ) -> Self::Args {
                [
                    $(
                        word
                            .next()
                            .unwrap_or_else(|| {
                                panic!(
                                    "Insufficient fields in event '{}': expected {}, found {}",
                                     $event_name,
                                     Self::FIELD_COUNT,
                                     $index,
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
                    $(,
                        word_eol
                            .nth($eol_index)
                            .unwrap_or_else(|| {
                                panic!(
                                    "Insufficient fields in event '{}': expected {}, found {}",
                                     $event_name,
                                     Self::FIELD_COUNT,
                                     $eol_index,
                                 )
                            })
                            .to_str()
                            .unwrap_or_else(|e| {
                                panic!(
                                    "Invalid UTF8 in field index {} of event '{}': {}",
                                    $eol_index,
                                    $event_name,
                                    e,
                                )
                            })
                    )?
                ]
            }
        }
    };
}

pub mod print;

pub mod server;