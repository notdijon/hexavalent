//! Traits for working with C-style null-terminated strings.

use std::ffi::{CStr, CString};

/// Used to convert various Rust strings into C-style strings that HexChat understands.
///
/// This conversion may or may not allocate, depending on the type and value.
/// Doc comments on each implementation indicate when it allocates.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait IntoCstr: private::IntoCstrImpl {}

/// Convert `&str` to `&CStr`.
///
/// This conversion allocates if the input string does not end with a null byte.
/// For example, `"hello"` would allocate, and `"hello\0"` would not allocate.
///
/// # Panics
///
/// If the input string contains interior null bytes.
/// For example, `"hel\0lo"` would panic, while `"hello"` and `"hello\0"` would not panic.
impl<'a> IntoCstr for &'a str {}

/// Convert `String` to `&CStr`.
///
/// This conversion allocates if the input string does not end with a null byte.
/// For example, `"hello"` would allocate, and `"hello\0"` would not allocate.
///
/// # Panics
///
/// If the input string contains interior null bytes.
/// For example, `"hel\0lo"` would panic, while `"hello"` and `"hello\0"` would not panic.
impl IntoCstr for String {}

/// Convert `&CStr` to `&CStr`.
///
/// This conversion never allocates.
impl<'a> IntoCstr for &'a CStr {}

/// Convert `CString` to `&CStr`.
///
/// This conversion never allocates.
impl IntoCstr for CString {}

mod private {
    use std::borrow::Cow;
    use std::ffi::{CStr, CString};

    pub trait IntoCstrImpl: Sized {
        type OWNER: AsRef<CStr>;

        fn into_cstr_owner(self) -> Self::OWNER;

        fn with_cstr<R>(self, f: impl FnOnce(&CStr) -> R) -> R {
            let owner = self.into_cstr_owner();
            f(owner.as_ref())
        }
    }

    impl<'a> IntoCstrImpl for &'a str {
        type OWNER = Cow<'a, CStr>;

        fn into_cstr_owner(self) -> Self::OWNER {
            // check last byte up front to avoid scanning the string twice if it does not end with null
            if self.as_bytes().last() == Some(&0) {
                Cow::Borrowed(CStr::from_bytes_with_nul(self.as_bytes()).unwrap())
            } else {
                Cow::Owned(CString::new(self).unwrap())
            }
        }

        fn with_cstr<R>(self, f: impl FnOnce(&CStr) -> R) -> R {
            // check last byte up front to avoid scanning the string twice if it does not end with null
            if self.as_bytes().last() == Some(&0) {
                f(CStr::from_bytes_with_nul(self.as_bytes()).unwrap())
            } else {
                f(CString::new(self).unwrap().as_ref())
            }
        }
    }

    impl IntoCstrImpl for String {
        type OWNER = CString;

        fn into_cstr_owner(self) -> Self::OWNER {
            let bytes = self.into_bytes();
            let first_null_byte = bytes.iter().position(|x| *x == 0);

            if first_null_byte == Some(bytes.len() - 1) {
                // Safety: `bytes` contains only one null byte in the last position
                unsafe {
                    let mut bytes = bytes;
                    bytes.pop();
                    CString::from_vec_unchecked(bytes)
                }
            } else if first_null_byte == None {
                // Safety: `bytes` contains no null bytes
                unsafe { CString::from_vec_unchecked(bytes) }
            } else {
                // This will always fail, but call `new` for a consistent error message
                CString::new(bytes).unwrap()
            }
        }

        fn with_cstr<R>(self, f: impl FnOnce(&CStr) -> R) -> R {
            // check last byte up front to avoid scanning the string twice if it does not end with null
            if self.as_bytes().last() == Some(&0) {
                f(CStr::from_bytes_with_nul(self.as_bytes()).unwrap())
            } else {
                f(CString::new(self).unwrap().as_ref())
            }
        }
    }

    impl<'a> IntoCstrImpl for &'a CStr {
        type OWNER = Self;

        fn into_cstr_owner(self) -> Self::OWNER {
            self
        }
    }

    impl<'a> IntoCstrImpl for CString {
        type OWNER = Self;

        fn into_cstr_owner(self) -> Self::OWNER {
            self
        }
    }
}

pub(crate) use private::IntoCstrImpl;
