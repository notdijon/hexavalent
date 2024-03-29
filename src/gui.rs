//! Fake plugins.

use std::ffi::c_void;
use std::ptr::NonNull;

/// A handle to a fake plugin in HexChat.
///
/// Returned from [`PluginHandle::plugingui_add`](crate::PluginHandle::plugingui_add).
///
/// Must be passed to [`PluginHandle::plugingui_remove`](crate::PluginHandle::plugingui_remove)
/// to remove the fake plugin.
#[must_use = "fake plugins are not removed automatically, you must call `plugingui_remove` yourself"]
#[derive(Debug)]
pub struct FakePluginHandle {
    /// Always holds a valid pointer returned by `hexchat_plugingui_add`
    handle: NonNull<c_void>,
}

impl FakePluginHandle {
    /// Creates a new `FakePluginHandle` from a pointer returned from `hexchat_plugingui_add`.
    ///
    /// # Safety
    ///
    /// `gui_handle` must have been returned from `hexchat_plugingui_add`.
    ///
    /// This function takes ownership of `gui_handle`; it must not be used afterwards.
    pub(crate) unsafe fn new(gui_handle: NonNull<c_void>) -> Self {
        Self { handle: gui_handle }
    }

    /// Converts this `FakePluginHandle` back into a raw pointer.
    pub(crate) fn into_raw(self) -> NonNull<c_void> {
        self.handle
    }
}
