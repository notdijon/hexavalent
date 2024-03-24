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

    /// Copies this `EventAttrs` instance and sets its timestamp.
    pub fn with_time(self, time: OffsetDateTime) -> Self {
        Self { time, ..self }
    }

    /// Copies this `EventAttrs` instance and sets its IRCv3 line.
    #[cfg(feature = "__unstable_ircv3_line_in_event_attrs")]
    pub fn with_ircv3_line(self, ircv3_line: &'a str) -> Self {
        Self { ircv3_line, ..self }
    }
}

/// Trait implemented by all event types.
///
/// See the [`PrintEvent`](print::PrintEvent) and [`ServerEvent`](server::ServerEvent) traits for usage.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Event<const ARGS: usize>: Default + private::EventImpl<ARGS> {}

pub(crate) mod private {
    use std::ffi::CStr;

    use crate::str::HexStr;

    pub trait EventImpl<const ARGS: usize> {
        const NAME: &'static CStr;

        /// Converts an array of C-style strings to this event's args.
        ///
        /// # Panics
        ///
        /// If `word` or `word_eol` contains invalid UTF8.
        fn args_from_words<'a>(
            word: impl Iterator<Item = &'a HexStr>,
            word_eol: impl Iterator<Item = &'a HexStr>,
        ) -> [&'a HexStr; ARGS];
    }
}

// This uses individual cases returning literals because rustdoc generates
// really bad output for macros returning expressions--it will just show the raw macro invocation,
// which is completely useless to a user. But if the macro returns a literal, it will show that.
macro_rules! count {
    () => {
        0
    };
    ($a:tt) => {
        1
    };
    ($a:tt $b:tt) => {
        2
    };
    ($a:tt $b:tt $c:tt) => {
        3
    };
    ($a:tt $b:tt $c:tt $d:tt) => {
        4
    };
    ($a:tt $b:tt $c:tt $d:tt $e:tt) => {
        5
    };
}

macro_rules! event {
    (
        $struct_name:ident,
        $event_name:literal,
        $event_doc:literal,
        $($index:tt : $field_name:literal),*
        $(; eol $eol_index:tt : $eol_name:literal)?
    ) => {
        #[doc = "`"]
        #[doc = $event_name]
        #[doc = "`"]
        #[doc = ""]
        #[doc = $event_doc]
        #[doc = ""]
        #[doc = "# Fields"]
        $(
            #[doc = ""]
            #[doc = "- `"]
            #[doc = $field_name]
            #[doc = "`"]
        )*
        $(
            #[doc = ""]
            #[doc = "- `"]
            #[doc = $eol_name]
            #[doc = "`"]
        )?
        #[derive(Debug, Copy, Clone, Default)]
        pub struct $struct_name;

        impl crate::event::Event<{ count!($($index)* $($eol_index)?) }> for $struct_name {}

        impl crate::event::private::EventImpl<{ count!($($index)* $($eol_index)?) }> for $struct_name {
            const NAME: &'static ::std::ffi::CStr = match ::std::ffi::CStr::from_bytes_with_nul(concat!($event_name, "\0").as_bytes()) {
                Ok(name) => name,
                Err(_) => unreachable!(),
            };

            #[allow(dead_code)]
            #[allow(unused_variables)]
            #[allow(unused_mut)]
            fn args_from_words<'a>(
                mut word: impl Iterator<Item = &'a crate::str::HexStr>,
                mut word_eol: impl Iterator<Item = &'a crate::str::HexStr>,
            ) -> [&'a crate::str::HexStr; { count!($($index)* $($eol_index)?) }] {
                const ARGS: usize = count!($($index)* $($eol_index)?);

                [
                    $(
                        word
                            .next()
                            .unwrap_or_else(|| {
                                panic!(
                                    "Insufficient fields in event '{}': expected {}, found {}",
                                     $event_name,
                                     ARGS,
                                     $index,
                                 )
                            }),
                    )*
                    $(
                        word_eol
                            .nth($eol_index)
                            .unwrap_or_else(|| {
                                panic!(
                                    "Insufficient fields in event '{}': expected {}, found {}",
                                     $event_name,
                                     ARGS,
                                     $eol_index,
                                 )
                            }),
                    )?
                ]
            }
        }
    };
}

pub mod print;

pub mod server;
