//! Types related to context/config info.

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

    pub enum PrefValue {
        String(String),
        Int(i32),
        Bool(bool),
    }

    pub trait FromPrefValue: Sized {
        fn from_pref_value(pref: PrefValue) -> Result<Self, ()>;
    }

    impl FromPrefValue for String {
        fn from_pref_value(pref: PrefValue) -> Result<Self, ()> {
            match pref {
                PrefValue::String(x) => Ok(x),
                _ => Err(()),
            }
        }
    }

    impl FromPrefValue for i32 {
        fn from_pref_value(pref: PrefValue) -> Result<Self, ()> {
            match pref {
                PrefValue::Int(x) => Ok(x),
                _ => Err(()),
            }
        }
    }

    impl FromPrefValue for bool {
        fn from_pref_value(pref: PrefValue) -> Result<Self, ()> {
            match pref {
                PrefValue::Bool(x) => Ok(x),
                _ => Err(()),
            }
        }
    }
}

pub(crate) use private::{FromPrefValue, PrefValue};

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

/// Global preferences.
pub mod prefs;

/// Special global preferences that do not appear in `/set`.
///
/// Analogous to the special preferences documented for [`hexchat_get_prefs`](https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_get_prefs).
pub mod prefs_special;
