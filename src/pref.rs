//! Global preferences.

/// The value of a HexChat setting.
///
/// Used with [`PluginHandle::get_pref`](crate::PluginHandle::get_pref).
///
/// Note that this represents a global preference, not a plugin-specific preference.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Pref: private::PrefImpl + 'static
where
    Self::Type: private::FromPrefValue,
{
    /// The preference's type.
    ///
    /// Can be `String`, `i32`, or `bool`.
    // todo with GATs, it _might_ be nice to have Type/BorrowedType<'a>, so that we can avoid allocation
    //  (but we'd probably have to make get_pref_with unsafe due to invalidation of the string)
    type Type: 'static;
}

pub(crate) mod private {
    use std::ffi::CStr;

    pub trait PrefImpl {
        const NAME: &'static CStr;
    }

    #[allow(unreachable_pub)]
    #[derive(Debug)]
    pub enum PrefValue<'a> {
        Str(&'a str),
        Int(i32),
        Bool(bool),
    }

    #[allow(unreachable_pub)]
    pub trait FromPrefValue: Sized {
        fn from_pref_value(pref: PrefValue<'_>) -> Result<Self, ()>;
    }
}

impl private::FromPrefValue for String {
    fn from_pref_value(pref: private::PrefValue<'_>) -> Result<Self, ()> {
        match pref {
            private::PrefValue::Str(x) => Ok(x.to_owned()),
            _ => Err(()),
        }
    }
}

impl private::FromPrefValue for i32 {
    fn from_pref_value(pref: private::PrefValue<'_>) -> Result<Self, ()> {
        match pref {
            private::PrefValue::Int(x) => Ok(x),
            _ => Err(()),
        }
    }
}

impl private::FromPrefValue for bool {
    fn from_pref_value(pref: private::PrefValue<'_>) -> Result<Self, ()> {
        match pref {
            private::PrefValue::Bool(x) => Ok(x),
            _ => Err(()),
        }
    }
}

macro_rules! pref {
    ($struct_name:ident, $pref_name:literal, $ty:ty) => {
        #[doc = "`"]
        #[doc = $pref_name]
        #[doc = "`"]
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name;

        impl crate::pref::private::PrefImpl for $struct_name {
            const NAME: &'static ::std::ffi::CStr =
                match ::std::ffi::CStr::from_bytes_with_nul(concat!($pref_name, "\0").as_bytes()) {
                    Ok(name) => name,
                    Err(_) => unreachable!(),
                };
        }

        impl crate::pref::Pref for $struct_name {
            type Type = $ty;
        }
    };
}

mod impls;

pub use impls::*;

/// Special global preferences that do not appear in `/set`.
///
/// Analogous to the special preferences documented for [`hexchat_get_prefs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_get_prefs).
pub mod special;
