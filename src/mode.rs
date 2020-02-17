//! Types related to sending modes.

/// Whether to add or remove a mode.
///
/// Used with [`PluginHandle::send_modes`](../struct.PluginHandle.html#method.send_modes).
pub enum Sign {
    /// Add the mode.
    Add,
    /// Remove the mode.
    Remove,
}
