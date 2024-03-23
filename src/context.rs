//! Server/channel contexts.

use std::marker::PhantomData;
use std::ptr::NonNull;

use crate::cstr::IntoCStr;
use crate::ffi::hexchat_context;

/// Criteria used to find a server/channel context.
///
/// Used with [`PluginHandle::find_context`](crate::PluginHandle::find_context).
///
/// Analogous to arguments passed to [`hexchat_find_context`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_find_context).
#[derive(Debug)]
pub struct Context<S: IntoCStr> {
    pub(crate) servname: Option<S>,
    pub(crate) channel: Option<S>,
}

impl Context<String> {
    /// The currently-focused tab/window.
    pub fn focused() -> Self {
        Self {
            servname: None,
            channel: None,
        }
    }
}

impl<S: IntoCStr> Context<S> {
    /// The specified channel in the current server, or if no matching channel exists, in any server.
    ///
    /// This is usually what you want for responding to server events,
    /// since the context in your hook callback will already be in the correct server.
    ///
    /// The channel name should include the leading `#` where present.
    pub fn channel(channel: S) -> Self {
        Self {
            servname: None,
            channel: Some(channel),
        }
    }

    /// The frontmost channel in the specified server.
    pub fn frontmost(servname: S) -> Self {
        Self {
            servname: Some(servname),
            channel: None,
        }
    }

    /// The specified channel in the specified server.
    ///
    /// It is generally not necessary to use this over [`Context::channel`](Context::channel),
    /// unless you need to print messages to a server in response to actions in a different server.
    ///
    /// The channel name should include the leading `#` where present.
    pub fn fully_qualified(servname: S, channel: S) -> Self {
        Self {
            servname: Some(servname),
            channel: Some(channel),
        }
    }
}

/// A handle to a server/channel context in HexChat.
///
/// Returned from [`PluginHandle::find_context`](crate::PluginHandle::find_context).
///
/// Should be passed to [`PluginHandle::with_context`](crate::PluginHandle::with_context) to run code in the context.
#[derive(Debug, Copy, Clone)]
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
    pub(crate) fn into_raw(self) -> NonNull<hexchat_context> {
        self.handle
    }
}
