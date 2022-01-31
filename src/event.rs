//! Print and server events.

use std::marker::PhantomData;

use time::OffsetDateTime;

/// Attributes associated with an event.
///
/// Used with [`PluginHandle::emit_print_attrs`](crate::PluginHandle::emit_print_attrs),
/// [`PluginHandle::hook_print_attrs`](crate::PluginHandle::hook_print_attrs),
/// and [`PluginHandle::hook_server_attrs`](crate::PluginHandle::hook_server_attrs).
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
/// See the [`PrintEvent`](print::PrintEvent) and [`ServerEvent`](server::ServerEvent) traits for usage.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Event<'a>: private::EventImpl<'a>
where
    Self::Args: From<Self::ArgsImpl>,
    Self::Args: Into<Self::ArgsImpl>,
{
    /// The arguments associated with this event.
    type Args: AsRef<[&'a str]>;
}

pub(crate) mod private {
    use std::ffi::CStr;
    use std::os::raw::c_char;

    /// Underlying private event implementation.
    ///
    /// # Safety
    ///
    /// See safety comments on each member.
    pub unsafe trait EventImpl<'a> {
        /// The arguments associated with this event.
        ///
        /// Should be the same as `<Self as Event<'a>::Args`.
        type ArgsImpl;

        /// The event's name.
        ///
        /// # Safety
        ///
        /// Must point to a valid, null-terminated C-style string.
        const NAME: *const c_char;

        /// Converts this event's args to C-style strings.
        fn args_to_c<R>(args: impl Into<Self::ArgsImpl>, f: impl FnOnce(&[&CStr]) -> R) -> R;

        /// Converts an array of C-style strings to this event's args.
        ///
        /// # Panics
        ///
        /// If `word` or `word_eol` contains invalid UTF8.
        fn args_from_words<R: From<Self::ArgsImpl>>(
            word: impl Iterator<Item = &'a ::std::ffi::CStr>,
            word_eol: impl Iterator<Item = &'a ::std::ffi::CStr>,
        ) -> R;
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
        }

        unsafe impl<'a> crate::event::private::EventImpl<'a> for $struct_name {
            type ArgsImpl = [&'a str; Self::FIELD_COUNT];

            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($event_name, "\0").as_ptr().cast();

            #[allow(unused_variables)]
            fn args_to_c<R>(args: impl Into<Self::ArgsImpl>, f: impl FnOnce(&[&::std::ffi::CStr]) -> R) -> R {
                let args: Self::ArgsImpl = args.into();
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

            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn args_from_words<R: From<Self::ArgsImpl>>(
                mut word: impl Iterator<Item = &'a ::std::ffi::CStr>,
                mut word_eol: impl Iterator<Item = &'a ::std::ffi::CStr>,
            ) -> R {
                let args = [
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
                ];
                R::from(args)
            }
        }
    };
}

pub mod print;

pub mod server;
