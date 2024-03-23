//! Conversion to C strings.

use std::ffi::{CStr, CString};

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
pub trait IntoCStr: private::IntoCStrImpl {}

pub(crate) mod private {
    use std::ffi::CStr;
    use std::ops::Deref;

    pub trait IntoCStrImpl {
        type CSTR: Deref<Target = CStr>;

        fn into_cstr(self) -> Self::CSTR;
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
