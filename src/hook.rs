//! Hook callbacks.

use std::ptr::NonNull;

use crate::ffi::hexchat_hook;
use crate::ffi::{
    HEXCHAT_EAT_ALL, HEXCHAT_EAT_HEXCHAT, HEXCHAT_EAT_NONE, HEXCHAT_EAT_PLUGIN, HEXCHAT_PRI_HIGH,
    HEXCHAT_PRI_HIGHEST, HEXCHAT_PRI_LOW, HEXCHAT_PRI_LOWEST, HEXCHAT_PRI_NORM,
};

/// Determines the order in which hook callbacks are called.
///
/// Used with hook registration functions such as [`PluginHandle::hook_command`](crate::PluginHandle::hook_command).
///
/// Unless you need to intercept events in a certain order, use  `Priority::Normal`.
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum Priority {
    /// Callbacks with the lowest priority run after callbacks with any other priority.
    ///
    /// Analogous to [`HEXCHAT_PRI_LOWEST`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_PRI_LOWEST).
    Lowest = HEXCHAT_PRI_LOWEST as isize,
    /// Analogous to [`HEXCHAT_PRI_LOW`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_PRI_LOW).
    Low = HEXCHAT_PRI_LOW as isize,
    /// Most callbacks should use normal priority.
    ///
    /// Analogous to [`HEXCHAT_PRI_NORM`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_PRI_NORM).
    Normal = HEXCHAT_PRI_NORM as isize,
    /// Analogous to [`HEXCHAT_PRI_HIGH`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_PRI_HIGH).
    High = HEXCHAT_PRI_HIGH as isize,
    /// Callbacks with the highest priority run before callbacks with any other priority.
    ///
    /// Analogous to [`HEXCHAT_PRI_HIGHEST`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_PRI_HIGHEST).
    Highest = HEXCHAT_PRI_HIGHEST as isize,
}

/// Whether the event that triggered a hook callback should be "eaten".
///
/// Used with hook registration functions such as [`PluginHandle::hook_command`](crate::PluginHandle::hook_command).
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum Eat {
    /// Let this event continue uneaten.
    ///
    /// Analogous to [`HEXCHAT_EAT_NONE`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_EAT_NONE).
    None = HEXCHAT_EAT_NONE as isize,
    /// Prevent this event from reaching HexChat.
    ///
    /// Analogous to [`HEXCHAT_EAT_HEXCHAT`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_EAT_XCHAT).
    HexChat = HEXCHAT_EAT_HEXCHAT as isize,
    /// Prevent this event from reaching other plugin callbacks.
    ///
    /// Analogous to [`HEXCHAT_EAT_PLUGIN`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_EAT_PLUGIN).
    Plugin = HEXCHAT_EAT_PLUGIN as isize,
    /// Prevent this event from reaching HexChat or other plugin callbacks.
    ///
    /// Analogous to [`HEXCHAT_EAT_ALL`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.HEXCHAT_EAT_ALL).
    All = HEXCHAT_EAT_ALL as isize,
}

/// Whether a timer callback should continue running.
///
/// Used with [`PluginHandle::hook_timer`](crate::PluginHandle::hook_timer).
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum Timer {
    /// Keep running the timer callback on the specified interval.
    // "return 1 to keep running" https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_hook_timer
    Continue = 1,
    /// Stop running the timer callback.
    Stop = 0,
}

/// A handle to a hook registered with HexChat.
///
/// Cannot be constructed in user code, but is returned from hook registration functions such as
/// [`PluginHandle::hook_command`](crate::PluginHandle::hook_command).
///
/// Can be passed to [`PluginHandle::unhook`](crate::PluginHandle::unhook) to unregister the hook,
/// although this is rarely necessary.
///
/// HexChat automatically unhooks any remaining hooks after your plugin finishes unloading,
/// so this type is only useful if you need to unhook a hook while your plugin is running.
///
/// # Examples
///
/// ```rust
/// use std::cell::Cell;
/// use hexavalent::{Plugin, PluginHandle};
/// use hexavalent::hook::{Eat, HookHandle, Priority};
///
/// #[derive(Default)]
/// struct MyPlugin {
///     cmd_handle: Cell<Option<HookHandle>>,
/// }
///
/// impl Plugin for MyPlugin {
///     fn init(&self, ph: PluginHandle<'_, Self>) {
///         let hook = ph.hook_command(
///             "theCommand\0",
///             "Usage: THECOMMAND, can be disabled\0",
///             Priority::Normal,
///             |plugin, ph, words| {
///                 ph.print("Yep, it still works.\0");
///                 Eat::All
///             }
///         );
///         self.cmd_handle.set(Some(hook));
///
///         ph.hook_command(
///             "disableTheCommand\0",
///             "Usage: DISABLETHECOMMAND, disables /theCommand\0",
///             Priority::Normal,
///             |plugin, ph, words| {
///                 match plugin.cmd_handle.take() {
///                     Some(hook) => {
///                         ph.unhook(hook);
///                         ph.print("Disabled the command!\0");
///                     }
///                     None => {
///                         ph.print("Command already disabled!\0");
///                     }
///                 }
///                 Eat::All
///             }
///         );
///     }
/// }
/// ```
#[derive(Debug)]
pub struct HookHandle {
    /// Always points to a valid instance of `hexchat_hook`
    handle: NonNull<hexchat_hook>,
}

impl HookHandle {
    /// Creates a new `HookHandle` from a native `hexchat_hook`.
    ///
    /// # Safety
    ///
    /// `hook_handle` must point to a valid instance of `hexchat_hook`.
    ///
    /// This function takes ownership of `hook_handle`; it must not be used afterwards.
    pub(crate) unsafe fn new(hook_handle: NonNull<hexchat_hook>) -> Self {
        Self {
            handle: hook_handle,
        }
    }

    /// Converts this `HookHandle` back into a native `hexchat_hook`.
    pub(crate) fn into_raw(self) -> NonNull<hexchat_hook> {
        self.handle
    }
}

#[cfg(test)]
mod tests {
    use std::mem;

    use super::*;

    #[test]
    fn one_byte_enums() {
        assert_eq!(mem::size_of::<Priority>(), 1);
        assert_eq!(mem::size_of::<Eat>(), 1);
        assert_eq!(mem::size_of::<Timer>(), 1);
    }
}
