//! Sending modes.

/// Whether to add or remove a mode.
///
/// Used with [`PluginHandle::send_modes`](crate::PluginHandle::send_modes).
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum Sign {
    /// Add the mode.
    Add,
    /// Remove the mode.
    Remove,
}
