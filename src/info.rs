//! Types related to context/config info.

/// Info about the current [context](../struct.PluginHandle.html#impl-3).
///
/// Used with [`PluginHandle::get_info`](../struct.PluginHandle.html#method.get_info).
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

    impl FromInfoValue for String {
        fn from_info_value(info: Option<&str>) -> Self {
            info.map(ToOwned::to_owned)
                .unwrap_or_else(|| panic!("Unexpected null info value"))
        }
    }

    impl FromInfoValue for Option<String> {
        fn from_info_value(info: Option<&str>) -> Self {
            info.map(ToOwned::to_owned)
        }
    }

    pub unsafe trait PrefImpl {
        /// The preference's name.
        ///
        /// # Safety
        ///
        /// Must point to a valid, null-terminated C-style string.
        const NAME: *const c_char;
    }

    pub enum PrefValue<'a> {
        Str(&'a str),
        Int(i32),
        Bool(bool),
    }

    pub trait FromPrefValue: Sized {
        fn from_pref_value(pref: PrefValue<'_>) -> Result<Self, ()>;
    }

    impl FromPrefValue for String {
        fn from_pref_value(pref: PrefValue<'_>) -> Result<Self, ()> {
            match pref {
                PrefValue::Str(x) => Ok(x.to_owned()),
                _ => Err(()),
            }
        }
    }

    impl FromPrefValue for i32 {
        fn from_pref_value(pref: PrefValue<'_>) -> Result<Self, ()> {
            match pref {
                PrefValue::Int(x) => Ok(x),
                _ => Err(()),
            }
        }
    }

    impl FromPrefValue for bool {
        fn from_pref_value(pref: PrefValue<'_>) -> Result<Self, ()> {
            match pref {
                PrefValue::Bool(x) => Ok(x),
                _ => Err(()),
            }
        }
    }
}

pub(crate) use private::{FromInfoValue, FromPrefValue, PrefValue};

macro_rules! info {
    ($struct_name:ident, $info_name:literal, $ty:ty, $description:literal) => {
        #[doc = "`"]
        #[doc = $info_name]
        #[doc = "`"]
        #[doc = ""]
        #[doc = $description]
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

/// Information types.
pub mod types;

macro_rules! pref {
    ($struct_name:ident, $pref_name:literal, $ty:ty) => {
        #[doc = "`"]
        #[doc = $pref_name]
        #[doc = "`"]
        pub struct $struct_name;

        unsafe impl crate::info::private::PrefImpl for $struct_name {
            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($pref_name, "\0").as_ptr().cast();
        }

        impl crate::info::Pref for $struct_name {
            type Type = $ty;
        }
    };
}

/// Global preference types.
pub mod prefs;

/// Special global preferences that do not appear in `/set`.
///
/// Analogous to the special preferences documented for [`hexchat_get_prefs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_get_prefs).
pub mod prefs_special;
