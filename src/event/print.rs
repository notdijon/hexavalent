//! Print event types.
//!
//! A list of all print events can also be viewed in HexChat under Settings > Text Events.

use crate::event::Event;

/// Trait implemented by all print event types.
///
/// Used with [`PluginHandle::emit_print`](crate::PluginHandle::emit_print),
/// [`PluginHandle::emit_print_attrs`](crate::PluginHandle::emit_print_attrs),
/// [`PluginHandle::hook_print`](crate::PluginHandle::hook_print),
/// and [`PluginHandle::hook_print_attrs`](crate::PluginHandle::hook_print_attrs).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
///
/// # Examples
///
/// Emitting a print event.
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::event::print::ChannelMessage;
///
/// fn print_welcome_message<P>(ph: PluginHandle<'_, P>) -> Result<(), ()> {
///     ph.emit_print(ChannelMessage, [c"hexavalent", c"Plugin started!", c"@", c""])
/// }
/// ```
///
/// Registering a hook for a print event.
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::event::print::ChannelMessage;
/// use hexavalent::hook::{Eat, Priority};
///
/// fn hook_message<P>(ph: PluginHandle<'_, P>) {
///     ph.hook_print(ChannelMessage, Priority::Normal, |plugin, ph, [nick, text, mode, ident]| {
///         ph.print(format!(
///             "Message from {} (with mode '{}', ident '{}'): {}",
///             nick, mode, ident, text
///         ));
///         Eat::HexChat
///     });
/// }
/// ```
pub trait PrintEvent<const ARGS: usize>: Event<ARGS> {}

macro_rules! print_event {
    (
        $struct_name:ident,
        $event_name:literal,
        $event_doc:literal,
        $($index:tt : $field_name:literal),*
    ) => {
        event!($struct_name, $event_name, $event_doc, $($index : $field_name),*);

        impl crate::event::print::PrintEvent<{ count!($($index)*) }> for $struct_name {}
    };
}

mod impls;

pub use impls::*;

/// Special print event types which can only be hooked, not emitted.
///
/// Attempting to emit these events with emission functions such as [`PluginHandle::emit_print`](crate::PluginHandle::emit_print) will always fail.
///
/// Analogous to the special print events documented for [`hexchat_hook_print`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_print).
pub mod special;
