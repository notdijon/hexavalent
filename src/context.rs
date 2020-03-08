//! Types related to server/channel contexts.

use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::ffi::hexchat_context;

/// Criteria used to find a server/channel context.
///
/// Used with [`PluginHandle::find_context`](../struct.PluginHandle.html#method.find_context).
///
/// Analogous to arguments passed to [`hexchat_find_context`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_find_context).
pub enum Context<'a> {
    /// The currently-focused tab/window.
    Focused,
    /// The specified channel in the current server, or if no matching channel exists, in any server.
    ///
    /// This is usually what you want for responding to server events,
    /// since the context in your hook callback will already be in the correct server.
    Nearby {
        /// The channel name, including the leading `#` where present.
        channel: &'a str,
    },
    /// The frontmost channel in the specified server.
    Frontmost {
        /// The user-friendly server name displayed by HexChat, e.g. `"Snoonet"`, _not_ a server URL.
        servname: &'a str,
    },
    /// The specified channel in the specified server.
    ///
    /// It is generally not necessary to use this variant over `Context::Nearby`,
    /// unless you need to print messages to a server in response to actions in a different server.
    FullyQualified {
        /// The user-friendly server name displayed by HexChat, e.g. `"Snoonet"`, _not_ a server URL.
        servname: &'a str,
        /// The channel name, including the leading `#` where present.
        channel: &'a str,
    },
}

/// A handle to a server/channel context in HexChat.
///
/// Cannot be constructed in user code, but is returned from
/// [`PluginHandle::find_context`](../struct.PluginHandle.html#method.find_context).
///
/// Can be passed to [`PluginHandle::with_context`](../struct.PluginHandle.html#method.with_context) to run code in the context.
#[derive(Copy, Clone)]
#[must_use = "context handles do nothing on their own, you must call `with_context` yourself"]
pub struct ContextHandle<'a> {
    handle: NonNull<hexchat_context>,
    _lifetime: PhantomData<&'a hexchat_context>,
}

impl<'a> ContextHandle<'a> {
    /// Creates a new `ContextHandle` from a native `hexchat_context`.
    ///
    /// # Safety
    ///
    /// `context_handle` must point to a valid instance of `hexchat_context`.
    pub(crate) unsafe fn new(context_handle: NonNull<hexchat_context>) -> Self {
        Self {
            handle: context_handle,
            _lifetime: PhantomData,
        }
    }

    /// Converts this `ContextHandle` back into a native `hexchat_context`.
    pub(crate) fn as_ptr(self) -> NonNull<hexchat_context> {
        self.handle
    }
}
