//! Context info.

use crate::str::{HexStr, HexString};

/// Info about the current [context](crate::PluginHandle::find_context).
///
/// Used with [`PluginHandle::get_info`](crate::PluginHandle::get_info).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Info: private::InfoImpl + 'static
where
    Self::Type: private::FromInfoValue,
{
    /// The info's type.
    ///
    /// Can be `String`, or `Option<String>`.
    // todo with GATs, it _might_ be nice to have Type/BorrowedType<'a>, so that we can avoid allocation
    //  (but we'd probably have to make get_info_with unsafe due to invalidation of the string)
    type Type: 'static;
}

pub(crate) mod private {
    use std::ffi::CStr;

    use crate::str::HexStr;

    pub trait InfoImpl {
        const NAME: &'static CStr;
    }

    #[allow(unreachable_pub)]
    pub trait FromInfoValue: Sized {
        fn from_info_value(info: Option<&HexStr>) -> Self;
    }
}

impl private::FromInfoValue for HexString {
    fn from_info_value(info: Option<&HexStr>) -> Self {
        info.map(ToOwned::to_owned)
            .unwrap_or_else(|| panic!("Unexpected null info value"))
    }
}

impl private::FromInfoValue for Option<HexString> {
    fn from_info_value(info: Option<&HexStr>) -> Self {
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

        impl crate::info::private::InfoImpl for $struct_name {
            const NAME: &'static ::std::ffi::CStr =
                match ::std::ffi::CStr::from_bytes_with_nul(concat!($info_name, "\0").as_bytes()) {
                    Ok(name) => name,
                    Err(_) => unreachable!(),
                };
        }

        impl crate::info::Info for $struct_name {
            type Type = $ty;
        }
    };
}

mod impls;

pub use impls::*;
