//! Conversion to and from C strings.

use std::borrow::Borrow;
use std::ffi::{CStr, CString};
use std::fmt::{self, Debug, Display};
use std::mem;
use std::ops::Deref;
use std::str::Utf8Error;

/// Converts various string types to C strings ([`CStr`]), which are required by HexChat.
///
/// Used with various HexChat functions that take strings, for example [`PluginHandle::print`](crate::PluginHandle::print).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
///
/// # Examples
///
/// Passing in a string as `&str` or `&CStr` will behave the same, but the former needs to allocate to add the null byte.
///
/// Strings that contain an interior null byte will panic.
///
/// ```rust
/// use hexavalent::PluginHandle;
///
/// fn print_some_stuff<P>(ph: PluginHandle<'_, P>) {
///     // for example, this would not allocate
///     ph.print(c"hello");
///     // ...this would allocate
///     ph.print("hello");
///     // ...and this would panic
///     ph.print("hel\0lo");
/// }
/// ```
#[allow(private_bounds)]
pub trait IntoCStr: private::IntoCStrImpl {}

/// Represents an `N`-element array or tuple of [`IntoCStr`] types.
///
/// Used with HexChat functions that emit events, for example [`PluginHandle::emit_print`](crate::PluginHandle::emit_print).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
///
/// # Examples
///
/// An array containing elements of some type that implements [`IntoCStr`]:
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::event::print::ChannelMessage;
///
/// fn emit_fake_print<P>(ph: PluginHandle<'_, P>) {
///     ph.emit_print(ChannelMessage, ["user", "hello", "@", "$"]);
/// }
/// ```
///
/// A tuple containing different types that implement [`IntoCStr`]:
///
/// ```rust
/// use hexavalent::PluginHandle;
/// use hexavalent::event::print::ChannelMessage;
///
/// fn emit_fake_print<P>(ph: PluginHandle<'_, P>, user: &str) {
///     ph.emit_print(ChannelMessage, (user, format!("hello from {user}"), c"@", c"$"));
/// }
/// ```
#[allow(private_bounds)]
pub trait IntoCStrArray<const N: usize>: private::IntoCStrArrayImpl<N> {}

pub(crate) mod private {
    use std::ffi::CStr;
    use std::ops::Deref;

    pub(crate) trait IntoCStrImpl {
        type CSTR: Deref<Target = CStr>;

        fn into_cstr(self) -> Self::CSTR;
    }

    /// Does the initial conversion from the tuple of `IntoCStr` types to a tuple of each type's `IntoCStr::CSTR` type.
    pub(crate) trait IntoCStrArrayImpl<const N: usize> {
        type CSTRS: AsCStrArray<N>;

        fn into_cstrs(self) -> Self::CSTRS;
    }

    /// Does the `Deref<Target=CStr>` mapping from each element of `IntoCStrArrayImpl` to an array of `&CStr`.
    pub(crate) trait AsCStrArray<const N: usize> {
        fn as_cstr_array(&self) -> [&CStr; N];
    }
}

impl IntoCStr for &str {}

impl IntoCStr for String {}

impl IntoCStr for &CStr {}

impl IntoCStr for CString {}

impl IntoCStr for &HexStr {}

impl IntoCStr for HexString {}

impl<'a> private::IntoCStrImpl for &'a str {
    type CSTR = CString;

    fn into_cstr(self) -> Self::CSTR {
        CString::new(self).unwrap()
    }
}

impl private::IntoCStrImpl for String {
    type CSTR = CString;

    fn into_cstr(self) -> Self::CSTR {
        CString::new(self).unwrap()
    }
}

impl<'a> private::IntoCStrImpl for &'a CStr {
    type CSTR = &'a CStr;

    fn into_cstr(self) -> Self::CSTR {
        self
    }
}

impl private::IntoCStrImpl for CString {
    type CSTR = CString;

    fn into_cstr(self) -> Self::CSTR {
        self
    }
}

impl<'a> private::IntoCStrImpl for &'a HexStr {
    type CSTR = &'a CStr;

    fn into_cstr(self) -> Self::CSTR {
        self.as_ref()
    }
}

impl private::IntoCStrImpl for HexString {
    type CSTR = CString;

    fn into_cstr(self) -> Self::CSTR {
        self.into_cstring()
    }
}

impl<S, const N: usize> IntoCStrArray<N> for [S; N] where S: IntoCStr {}

impl IntoCStrArray<0> for () {}
impl<A> IntoCStrArray<1> for (A,) where A: IntoCStr {}
impl<A, B> IntoCStrArray<2> for (A, B)
where
    A: IntoCStr,
    B: IntoCStr,
{
}
impl<A, B, C> IntoCStrArray<3> for (A, B, C)
where
    A: IntoCStr,
    B: IntoCStr,
    C: IntoCStr,
{
}
impl<A, B, C, D> IntoCStrArray<4> for (A, B, C, D)
where
    A: IntoCStr,
    B: IntoCStr,
    C: IntoCStr,
    D: IntoCStr,
{
}

impl<S: IntoCStr, const N: usize> private::IntoCStrArrayImpl<N> for [S; N] {
    type CSTRS = [S::CSTR; N];

    fn into_cstrs(self) -> Self::CSTRS {
        self.map(private::IntoCStrImpl::into_cstr)
    }
}

impl private::IntoCStrArrayImpl<0> for () {
    type CSTRS = ();

    fn into_cstrs(self) -> Self::CSTRS {}
}

impl<A: IntoCStr> private::IntoCStrArrayImpl<1> for (A,) {
    type CSTRS = (A::CSTR,);

    fn into_cstrs(self) -> Self::CSTRS {
        (self.0.into_cstr(),)
    }
}

impl<A: IntoCStr, B: IntoCStr> private::IntoCStrArrayImpl<2> for (A, B) {
    type CSTRS = (A::CSTR, B::CSTR);

    fn into_cstrs(self) -> Self::CSTRS {
        (self.0.into_cstr(), self.1.into_cstr())
    }
}

impl<A: IntoCStr, B: IntoCStr, C: IntoCStr> private::IntoCStrArrayImpl<3> for (A, B, C) {
    type CSTRS = (A::CSTR, B::CSTR, C::CSTR);

    fn into_cstrs(self) -> Self::CSTRS {
        (self.0.into_cstr(), self.1.into_cstr(), self.2.into_cstr())
    }
}

impl<A: IntoCStr, B: IntoCStr, C: IntoCStr, D: IntoCStr> private::IntoCStrArrayImpl<4>
    for (A, B, C, D)
{
    type CSTRS = (A::CSTR, B::CSTR, C::CSTR, D::CSTR);

    fn into_cstrs(self) -> Self::CSTRS {
        (
            self.0.into_cstr(),
            self.1.into_cstr(),
            self.2.into_cstr(),
            self.3.into_cstr(),
        )
    }
}

impl<S: Deref<Target = CStr>, const N: usize> private::AsCStrArray<N> for [S; N] {
    fn as_cstr_array(&self) -> [&CStr; N] {
        self.each_ref().map(Deref::deref)
    }
}

impl private::AsCStrArray<0> for () {
    fn as_cstr_array(&self) -> [&CStr; 0] {
        []
    }
}

impl<A: Deref<Target = CStr>> private::AsCStrArray<1> for (A,) {
    fn as_cstr_array(&self) -> [&CStr; 1] {
        [self.0.deref()]
    }
}

impl<A: Deref<Target = CStr>, B: Deref<Target = CStr>> private::AsCStrArray<2> for (A, B) {
    fn as_cstr_array(&self) -> [&CStr; 2] {
        [self.0.deref(), self.1.deref()]
    }
}

impl<A: Deref<Target = CStr>, B: Deref<Target = CStr>, C: Deref<Target = CStr>>
    private::AsCStrArray<3> for (A, B, C)
{
    fn as_cstr_array(&self) -> [&CStr; 3] {
        [self.0.deref(), self.1.deref(), self.2.deref()]
    }
}

impl<
        A: Deref<Target = CStr>,
        B: Deref<Target = CStr>,
        C: Deref<Target = CStr>,
        D: Deref<Target = CStr>,
    > private::AsCStrArray<4> for (A, B, C, D)
{
    fn as_cstr_array(&self) -> [&CStr; 4] {
        [
            self.0.deref(),
            self.1.deref(),
            self.2.deref(),
            self.3.deref(),
        ]
    }
}

/// A string slice returned from HexChat.
///
/// This type is very similar to [`&str`](str), except it's known to be returned from HexChat and thus null terminated.
/// This means it can be passed to any function that accepts an [`impl IntoCStr`](IntoCStr) argument,
/// and it will not require allocation like [`&str`](str).
///
/// `HexStr` [`derefs`](Deref) to a `&str` that doesn't include the trailing null byte,
/// so it can generally be used as a normal string slice. To do this conversion explicitly,
/// call [`as_str`](HexStr::as_str).
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct HexStr {
    /// Invariant 1: Always null-terminated.
    /// Invariant 2: Contains no interior null bytes.
    inner: str,
}

impl HexStr {
    pub(crate) const EMPTY: &'static HexStr = match HexStr::from_cstr(c"") {
        Ok(hex) => hex,
        Err(_) => unreachable!(),
    };

    /// Creates a new `HexStr` from a string slice.
    ///
    /// # Safety
    ///
    /// The string must be null-terminated and contain no interior null bytes.
    pub(crate) unsafe fn from_null_terminated_str(str: &str) -> &HexStr {
        // SAFETY: `HexStr` is a repr(transparent) wrapper over `str`, so transmuting is safe
        // SAFETY: Invariant 1 and Invariant 2 forwarded to caller.
        unsafe { mem::transmute(str) }
    }

    pub(crate) const fn from_cstr(cstr: &CStr) -> Result<&HexStr, Utf8Error> {
        let bytes = cstr.to_bytes_with_nul();
        let str: &str = match std::str::from_utf8(bytes) {
            Ok(str) => str,
            Err(err) => return Err(err),
        };
        // SAFETY: `HexStr` is a repr(transparent) wrapper over `str`, so transmuting is safe
        // SAFETY: the byte array we used to create the string was null-terminated, so this upholds the type's invariant 1
        // SAFETY: the string used to be a CStr, which cannot contain null bytes, so this upholds the type's invariant 2
        let hex: &HexStr = unsafe { mem::transmute(str) };
        Ok(hex)
    }

    /// Convert this `HexStr` to a string slice, _without_ the trailing null byte.
    pub fn as_str(&self) -> &str {
        self.deref()
    }

    /// Convert this `HexStr` to a [`CStr`].
    pub fn as_cstr(&self) -> &CStr {
        self.as_ref()
    }
}

impl Debug for HexStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: &str = self.as_ref();
        Debug::fmt(str, f)
    }
}

impl Display for HexStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: &str = self.as_ref();
        Display::fmt(str, f)
    }
}

impl Deref for HexStr {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        // SAFETY: due to the type's invariant, the string is null-terminated,
        // so the string is nonempty and the last byte can be removed without breaking UTF8.
        unsafe { self.inner.get_unchecked(0..self.inner.len() - 1) }
    }
}

impl AsRef<str> for HexStr {
    fn as_ref(&self) -> &str {
        self.deref()
    }
}

impl AsRef<CStr> for HexStr {
    fn as_ref(&self) -> &CStr {
        // SAFETY: due to the type's invariant, the string is null-terminated and contains no interior null bytes
        unsafe { CStr::from_bytes_with_nul_unchecked(self.inner.as_bytes()) }
    }
}

impl ToOwned for HexStr {
    type Owned = HexString;

    fn to_owned(&self) -> Self::Owned {
        // SAFETY: due to the type's invariant, the string is null-terminated and contains no interior null bytes
        unsafe { HexString::from_null_terminated_string(self.inner.to_owned()) }
    }
}

/// An owned string returned from HexChat.
///
/// This is the owned version of [`HexStr`] and behaves in the same way, acting like a [`String`]
/// but not requiring allocation to add a null byte when passed as [`impl IntoCStr`](IntoCStr).
///
/// `HexString` [`derefs`](Deref) to a `&str` that doesn't include the trailing null byte,
/// so it can generally be used as a normal string slice. To do this conversion explicitly,
/// call [`as_str`](HexStr::as_str).
///
/// Unlike [`String`], `HexString` is not mutable, as this could allow the null byte to be removed.
/// If you need mutability, call [`into_string`](HexString::into_string) to extract the underlying string
/// (with the trailing null byte removed).
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HexString {
    /// Invariant 1: Always null-terminated.
    /// Invariant 2: Contains no interior null bytes.
    inner: String,
}

impl HexString {
    /// Creates a new `HexString` from a `String`.
    ///
    /// # Safety
    ///
    /// The string must be null-terminated and contain no interior null bytes.
    pub(crate) unsafe fn from_null_terminated_string(string: String) -> HexString {
        // SAFETY: Invariant 1 and Invariant 2 forwarded to caller.
        HexString { inner: string }
    }

    /// Convert this `HexString` to a [`String`], _without_ the trailing null byte.
    pub fn into_string(self) -> String {
        let mut s = self.inner;
        let null = s.pop();
        debug_assert_eq!(null, Some('\0'));
        s
    }

    /// Convert this `HexString` to a [`CString`].
    pub fn into_cstring(self) -> CString {
        // SAFETY: due to the type's invariant, the string is null-terminated and contains no interior null bytes
        unsafe { CString::from_vec_with_nul_unchecked(self.inner.into_bytes()) }
    }
}

impl Debug for HexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: &str = self.as_ref();
        Debug::fmt(str, f)
    }
}

impl Display for HexString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str: &str = self.as_ref();
        Display::fmt(str, f)
    }
}

impl Deref for HexString {
    type Target = HexStr;

    fn deref(&self) -> &Self::Target {
        // SAFETY: due to the type's invariant, the string is null-terminated and contains no interior null bytes
        unsafe { HexStr::from_null_terminated_str(&self.inner) }
    }
}

impl Borrow<HexStr> for HexString {
    fn borrow(&self) -> &HexStr {
        self.deref()
    }
}

#[cfg(test)]
mod tests {
    use super::private::*;
    use super::*;

    #[test]
    fn intocstr_str() {
        let owner = "hello".into_cstr();
        assert_eq!(owner.as_ref(), c"hello");

        let owner = String::from("hello").into_cstr();
        assert_eq!(owner.as_ref(), c"hello");

        let owner = c"hello".into_cstr();
        assert_eq!(owner.as_ref(), c"hello");

        let owner = CString::from(c"hello").into_cstr();
        assert_eq!(owner.as_ref(), c"hello");
    }

    #[test]
    #[should_panic]
    fn intocstr_str_invalid_no_null() {
        "hel\0lo".into_cstr();
    }

    #[test]
    #[should_panic]
    fn intocstr_str_invalid_with_null() {
        "hel\0lo\0".into_cstr();
    }

    #[test]
    fn hexstr_empty_is_empty() {
        assert_eq!(HexStr::EMPTY.as_str(), "");
        assert_eq!(HexStr::EMPTY.as_cstr(), c"");
    }

    #[test]
    fn hexstr_from_cstr() {
        let hex = HexStr::from_cstr(c"hello").unwrap();
        assert_eq!(hex.as_str(), "hello");
        assert_eq!(hex.as_cstr(), c"hello");
    }

    #[test]
    fn hexstr_from_cstr_invalid() {
        assert!(HexStr::from_cstr(c"hello\xcf").is_err());
    }

    #[test]
    fn hexstr_debug() {
        let hex = HexStr::from_cstr(c"hello").unwrap();
        assert_eq!(format!("{:?}", hex), "\"hello\"");
    }

    #[test]
    fn hexstr_display() {
        let hex = HexStr::from_cstr(c"hello").unwrap();
        assert_eq!(format!("{}", hex), "hello");
    }

    #[test]
    fn hexstr_deref() {
        let hex = HexStr::from_cstr(c"hello").unwrap();
        assert_eq!(hex.len(), "hello".len());
    }

    #[test]
    fn hexstr_to_owned() {
        let hex = HexStr::from_cstr(c"hello").unwrap();
        let owned: HexString = hex.to_owned();
        assert_eq!(owned.as_str(), "hello");
    }

    #[test]
    fn hexstring_into_string() {
        let hex: HexString = HexStr::from_cstr(c"hello").unwrap().to_owned();
        assert_eq!(hex.into_string(), "hello");
    }

    #[test]
    fn hexstring_into_cstring() {
        let hex: HexString = HexStr::from_cstr(c"hello").unwrap().to_owned();
        assert_eq!(hex.into_cstring().as_c_str(), c"hello");
    }

    #[test]
    fn hexstring_debug() {
        let hex: HexString = HexStr::from_cstr(c"hello").unwrap().to_owned();
        assert_eq!(format!("{:?}", hex), "\"hello\"");
    }

    #[test]
    fn hexstring_display() {
        let hex: HexString = HexStr::from_cstr(c"hello").unwrap().to_owned();
        assert_eq!(format!("{}", hex), "hello");
    }

    #[test]
    fn hexstring_deref() {
        let hex: HexString = HexStr::from_cstr(c"hello").unwrap().to_owned();
        assert_eq!(hex.as_str(), "hello");
    }
}
