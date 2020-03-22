//! Global preferences.

/// The value of a HexChat setting.
///
/// Used with [`PluginHandle::get_pref`](../struct.PluginHandle.html#method.get_pref).
///
/// Note that this represents a global preference, not a plugin-specific preference.
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait Pref: private::PrefImpl {
    /// The preference's type.
    ///
    /// Can be `String`, `i32`, or `bool`.
    // todo with GATs, it _might_ be nice to have Type/BorrowedType<'a>, so that we can avoid allocation
    //  (but we'd probably have to make get_pref_with unsafe due to invalidation of the string)
    type Type: private::FromPrefValue;
}

pub(crate) mod private {
    use std::os::raw::c_char;

    pub unsafe trait PrefImpl {
        /// The preference's name.
        ///
        /// # Safety
        ///
        /// Must point to a valid, null-terminated C-style string.
        const NAME: *const c_char;
    }

    #[derive(Debug)]
    pub enum PrefValue<'a> {
        Str(&'a str),
        Int(i32),
        Bool(bool),
    }

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

        unsafe impl crate::pref::private::PrefImpl for $struct_name {
            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($pref_name, "\0").as_ptr().cast();
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
