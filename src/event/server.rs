//! Server event types.

use crate::event::Event;

/// Trait implemented by all server event types.
///
/// Used with [`PluginHandle::hook_server`](crate::PluginHandle::hook_server)
/// and [`PluginHandle::hook_server_attrs`](crate::PluginHandle::hook_server_attrs).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
///
/// # Examples
///
/// Registering a hook for a server event.
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::event::server::Privmsg;
/// use hexavalent::hook::{Eat, Priority};
///
/// fn hook_privmsg<P>(ph: PluginHandle<'_, P>) {
///     ph.hook_server(Privmsg, Priority::Normal, privmsg_cb);
/// }
///
/// fn privmsg_cb<P>(plugin: &P, ph: PluginHandle<'_, P>, args: [&str; 4]) -> Eat {
///     let [sender, _, target, text] = args;
///     ph.print(format!(
///         "Message from {} to {}: {}",
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
        $event_doc:literal,
        $($index:tt : $field_name:literal),*
        $(; eol $eol_index:tt : $eol_name:literal)?
    ) => {
        event!($struct_name, $event_name, $event_doc, $($index : $field_name),* $(; eol $eol_index : $eol_name)?);

        impl crate::event::server::ServerEvent for $struct_name {}
    };
}

mod impls;

pub use impls::*;

/// Special server events types which do not represent a message in the IRC specification.
///
/// Analogous to the special server events documented for [`hexchat_hook_server`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_server).
pub mod special;
