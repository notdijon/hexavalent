//! Types related to events.

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

/// Trait implemented by all event types.
///
/// See the [`PrintEvent`](trait.PrintEvent.html) and [`ServerEvent`](trait.ServerEvent.html) traits for usage.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Event<'a>: private::EventImpl {
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
        $event_format:literal,
        $($index:literal : $field_name:literal),*
        $(, eol $eol_index:literal : $eol_name:literal)?
    ) => {
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
        $(
            #[doc = "`"]
            #[doc = $eol_name]
            #[doc = "`, "]
        )?
        #[doc = "]."]
        pub struct $struct_name;

        impl $struct_name {
            const FIELD_COUNT: usize = count!($($index)* $($eol_index)?);
        }

        unsafe impl crate::events::private::EventImpl for $struct_name {
            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($event_name, "\0").as_ptr().cast();
        }

        impl<'a> crate::events::Event<'a> for $struct_name {
            #[doc = "["]
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
            #[doc = ""]
            #[doc = "]"]
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

/// Trait implemented by all print event types.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
///
/// # Examples
///
/// Emitting a print event.
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::events::print::ChannelMessage;
///
/// fn print_welcome_message<P>(ph: PluginHandle<'_, P>) -> Result<(), ()> {
///     ph.emit_print(ChannelMessage, ["hexavalent\0", "Plugin started!\0", "@\0", "\0"])
/// }
/// ```
///
/// Registering a hook for a print event.
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::events::{Event, PrintEvent};
/// use hexavalent::events::print::ChannelMessage;
/// use hexavalent::hook::{Eat, Priority};
///
/// fn hook_message<P: 'static>(ph: PluginHandle<'_, P>) {
///     ph.hook_print(ChannelMessage, Priority::Normal, message_cb);
/// }
///
/// fn message_cb<P>(
///     plugin: &P,
///     ph: PluginHandle<'_, P>,
///     args: <ChannelMessage as Event<'_>>::Args,
/// ) -> Eat {
///     let [nick, text, mode, ident] = args;
///     ph.print(&format!(
///         "Message from {} (with mode '{}', ident '{}'): {}\0",
///         nick, mode, ident, text
///     ));
///     Eat::HexChat
/// }
/// ```
pub trait PrintEvent: for<'a> Event<'a> {}

macro_rules! print_event {
    (
        $struct_name:ident,
        $event_name:literal,
        $event_format:literal,
        $($index:literal : $field_name:literal),*
    ) => {
        event!($struct_name, $event_name, $event_format, $($index : $field_name),*);

        impl crate::events::PrintEvent for $struct_name {}
    };
}

/// Print event types.
///
/// A list of all print events can also be viewed in HexChat under Settings > Text Events.
pub mod print;

/// Special print event types which can only be hooked, not emitted.
///
/// Used with hook registration functions such as [`PluginHandle::hook_print`](../../struct.PluginHandle.html#method.hook_print).
///
/// Attempting to emit these events with emission functions such as [`PluginHandle::emit_print`](../../struct.PluginHandle.html#method.emit_print) will always fail.
///
/// Analogous to the special print events documented for [`hexchat_hook_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_print).
pub mod print_special;

/// Trait implemented by all server event types.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
///
/// # Examples
///
/// Registering a hook for a server event.
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::events::{Event, ServerEvent};
/// use hexavalent::events::server::Privmsg;
/// use hexavalent::hook::{Eat, Priority};
///
/// fn hook_privmsg<P: 'static>(ph: PluginHandle<'_, P>) {
///     ph.hook_server(Privmsg, Priority::Normal, privmsg_cb);
/// }
///
/// fn privmsg_cb<P>(
///     plugin: &P,
///     ph: PluginHandle<'_, P>,
///     args: <Privmsg as Event<'_>>::Args,
/// ) -> Eat {
///     let [sender, _, target, text] = args;
///     ph.print(&format!(
///         "Message from {} to {}: {}\0",
///         sender, target, text
///     ));
///     Eat::None
/// }
/// ```
pub trait ServerEvent: for<'a> Event<'a> {}

macro_rules! server_event {
    (
        $struct_name:ident,
        $event_name:literal,
        $event_format:literal,
        $($index:literal : $field_name:literal),*
        $(, eol $eol_index:literal : $eol_name:literal)?
    ) => {
        event!($struct_name, $event_name, $event_format, $($index : $field_name),* $(, eol $eol_index : $eol_name)?);

        impl crate::events::ServerEvent for $struct_name {}
    };
}

/// Server event types.
pub mod server;

/// Special server events types which do not represent a message in the IRC specification.
///
/// Analogous to the special server events documented for [`hexchat_hook_server`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_server).
pub mod server_special;
