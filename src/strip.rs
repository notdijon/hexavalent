//! String format stripping.

use std::ffi::CStr;
use std::fmt::{self, Debug, Display, Formatter};
use std::ops::Deref;
use std::ptr::NonNull;
use std::str::Utf8Error;

use crate::ffi::RawPluginHandle;
use crate::str::HexStr;

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
/// Derefs to [`&HexStr`](crate::str::HexStr).
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
///     ph.print(format!("original: '{}' -> stripped: '{}'", orig, stripped));
/// }
/// ```
pub struct StrippedStr<'a> {
    raw: RawPluginHandle<'a>,
    /// Always points to a valid `HexStr`.
    stripped_ptr: NonNull<HexStr>,
}

impl<'a> StrippedStr<'a> {
    /// Create a `StrippedStr` from a stripped pointer returned by HexChat.
    ///
    /// # Safety
    ///
    /// `stripped_ptr` must point to a string returned by `hexchat_strip` which is valid for the entire lifetime `'a``.
    ///
    /// This function takes ownership of `stripped_ptr`; the underlying object must not be used afterwards.
    pub(crate) unsafe fn new(
        raw: RawPluginHandle<'a>,
        stripped_ptr: &CStr,
    ) -> Result<Self, Utf8Error> {
        let stripped_ptr = HexStr::from_cstr(stripped_ptr)?;
        Ok(Self {
            raw,
            stripped_ptr: NonNull::from(stripped_ptr),
        })
    }
}

impl Drop for StrippedStr<'_> {
    fn drop(&mut self) {
        // Safety: stripped_ptr was returned from hexchat_strip;
        //         we have conceptual ownership of stripped_str due to StrippedStr precondition
        unsafe {
            self.raw.hexchat_free(self.stripped_ptr.as_ptr().cast());
        }
    }
}

impl Deref for StrippedStr<'_> {
    type Target = HexStr;

    fn deref(&self) -> &Self::Target {
        // SAFETY: pointer is always valid.
        unsafe { self.stripped_ptr.as_ref() }
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
