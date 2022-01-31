//! String format stripping.

use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::os::raw::c_char;
use std::ptr::NonNull;
use std::slice;
use std::str;

use crate::ffi::RawPluginHandle;

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

/// A stripped string.
///
/// Derefs to `&str`.
///
/// Returned by [`PluginHandle::strip`](crate::PluginHandle::strip).
///
/// # Examples
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::strip::{MircColors, StrippedStr, TextAttrs};
///
/// fn strip_and_compare<P>(ph: PluginHandle<'_, P>, orig: &str) {
///     let stripped: Result<_, _> = ph.strip(orig, MircColors::Remove, TextAttrs::Remove);
///     let stripped = match &stripped {
///         Ok(s) => s,
///         Err(()) => "<failed to strip>",
///     };
///     ph.print(&format!("original: '{}' -> stripped: '{}'", orig, stripped));
/// }
/// ```
pub struct StrippedStr<'a> {
    raw: RawPluginHandle<'a>,
    stripped_ptr: NonNull<c_char>,
    len: usize,
}

impl<'a> StrippedStr<'a> {
    /// Create a `StrippedStr` from a stripped pointer returned by HexChat.
    ///
    /// # Safety
    ///
    /// `stripped_ptr` must point to `len` valid UTF8 bytes, originally returned by `hexchat_strip`.
    ///
    /// This function takes ownership of `stripped_ptr`; the underlying object must not be used afterwards.
    pub(crate) unsafe fn new(
        raw: RawPluginHandle<'a>,
        stripped_ptr: NonNull<c_char>,
        len: usize,
    ) -> Self {
        Self {
            raw,
            stripped_ptr,
            len,
        }
    }
}

impl Drop for StrippedStr<'_> {
    fn drop(&mut self) {
        // Safety: stripped_ptr was returned from hexchat_strip;
        //         we have conceptual ownership of stripped_str due to StrippedStr precondition
        unsafe {
            self.raw.hexchat_free(self.stripped_ptr.as_ptr() as *mut _);
        }
    }
}

impl Deref for StrippedStr<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // Safety: `stripped_ptr` points to `len` bytes
        let slice =
            unsafe { slice::from_raw_parts(self.stripped_ptr.as_ptr() as *const _, self.len) };
        // Safety: `stripped_ptr` points to valid UTF8
        unsafe { str::from_utf8_unchecked(slice) }
    }
}

impl AsRef<str> for StrippedStr<'_> {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl Display for StrippedStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.deref(), f)
    }
}

impl Debug for StrippedStr<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.deref(), f)
    }
}
