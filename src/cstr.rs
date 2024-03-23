//! Conversion to C strings.

use std::ffi::{CStr, CString};
use std::ops::Deref;

/// Converts various string types to C strings ([`CStr`]), which are required by Hexchat.
///
/// Used with various Hexchat functions that take strings, for example [`PluginHandle::print`](crate::PluginHandle::print).
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
/// Used with Hexchat functions that emit events, for example [`PluginHandle::emit_print`](crate::PluginHandle::emit_print).
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
}
