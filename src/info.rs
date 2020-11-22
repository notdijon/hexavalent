//! Context info.

/// Info about the current [context](crate::PluginHandle#impl-3).
///
/// Used with [`PluginHandle::get_info`](crate::PluginHandle::get_info).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Info: private::InfoImpl {
    /// The info's type.
    ///
    /// Can be `String`, or `Option<String>`.
    // todo with GATs, it _might_ be nice to have Type/BorrowedType<'a>, so that we can avoid allocation
    //  (but we'd probably have to make get_info_with unsafe due to invalidation of the string)
    type Type: private::FromInfoValue;
}

pub(crate) mod private {
    use std::os::raw::c_char;

    pub unsafe trait InfoImpl {
        /// The info's name.
        ///
        /// # Safety
        ///
        /// Must point to a valid, null-terminated C-style string.
        const NAME: *const c_char;
    }

    pub trait FromInfoValue: Sized {
        fn from_info_value(info: Option<&str>) -> Self;
    }
}

impl private::FromInfoValue for String {
    fn from_info_value(info: Option<&str>) -> Self {
        info.map(ToOwned::to_owned)
            .unwrap_or_else(|| panic!("Unexpected null info value"))
    }
}

impl private::FromInfoValue for Option<String> {
    fn from_info_value(info: Option<&str>) -> Self {
        info.map(ToOwned::to_owned)
    }
}

macro_rules! info {
    ($struct_name:ident, $info_name:literal, $ty:ty, $description:literal) => {
        #[doc = "`"]
        #[doc = $info_name]
        #[doc = "`"]
        #[doc = ""]
        #[doc = $description]
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name;

        unsafe impl crate::info::private::InfoImpl for $struct_name {
            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($info_name, "\0").as_ptr().cast();
        }

        impl crate::info::Info for $struct_name {
            type Type = $ty;
        }
    };
}

mod impls;

pub use impls::*;
