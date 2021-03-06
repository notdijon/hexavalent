//! String format stripping.

/// Whether to strip mIRC color attributes.
///
/// Used with [`PluginHandle::strip`](crate::PluginHandle::strip).
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum MircColors {
    /// Preserve mIRC colors.
    Keep,
    /// Strip mIRC colors.
    Remove,
}

/// Whether to strip text attributes (bold, underline, etc.).
///
/// Used with [`PluginHandle::strip`](crate::PluginHandle::strip).
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub enum TextAttrs {
    /// Preserve text attributes.
    Keep,
    /// Strip text attributes.
    Remove,
}
