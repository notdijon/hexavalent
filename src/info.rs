//! Types related to context/config info.

use std::convert::TryFrom;

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

/// A list that can be retrieved from HexChat.
///
/// Used with [`PluginHandle::get_list`](../struct.PluginHandle.html#method.get_list)
/// and [`PluginHandle::get_list_with`](../struct.PluginHandle.html#method.get_list_with).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait List: private::ListImpl {
    /// The type of elements of the list.
    // todo with GATs, it _might_ be nice to have Elem/BorrowedElem<'a>, so that we can avoid allocation
    //  (but we'd probably have to make get_list_with unsafe due to invalidation of the string)
    type Elem: private::FromListElem;
}

pub(crate) mod private {
    use std::os::raw::c_char;

    use crate::ffi::ListElem;

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

    pub unsafe trait ListImpl {
        /// The list's name.
        ///
        /// # Safety
        ///
        /// Must point to a valid, null-terminated C-style string.
        const NAME: *const c_char;
    }

    pub trait FromListElem: Sized {
        fn from_list_elem(elem: ListElem<'_>) -> Self;
    }
}

pub(crate) use private::{FromInfoValue, FromListElem, FromPrefValue, PrefValue};

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

macro_rules! list {
    (
        $struct_name:ident,
        $list_name:literal,
        $description:literal,
        $elem_desc:literal,
        $elem_ty:ident {
            $(
                [ $( $field_key:literal )? $( $custom:ident )?, $field_desc:literal, $( $field_type:ident )? $( |$elem:ident| $extract:expr )? ]
                $rust_field_name:ident : $rust_field_type:ty
            ),* $(,)?
        }
    ) => {
        #[doc = "`"]
        #[doc = $list_name]
        #[doc = "`"]
        #[doc = ""]
        #[doc = $description]
        pub struct $struct_name;

        unsafe impl crate::info::private::ListImpl for $struct_name {
            // Safety: this string is null-terminated and static
            const NAME: *const ::std::os::raw::c_char = concat!($list_name, "\0").as_ptr().cast();
        }

        impl crate::info::List for $struct_name {
            type Elem = $elem_ty;
        }

        #[doc = $elem_desc]
        ///
        /// See the [`List`](../trait.List.html) trait for usage.
        #[non_exhaustive]
        pub struct $elem_ty {
            $(
                #[doc = $field_desc]
                pub $rust_field_name : $rust_field_type
            ),*
        }

        impl crate::info::private::FromListElem for $elem_ty {
            fn from_list_elem(elem: crate::ffi::ListElem<'_>) -> Self {
                Self {
                    $(
                        $rust_field_name: {
                            let raw_value = list!(@generateFieldExtraction, elem, $( $field_key )? $( $custom )?, $( $field_type )? $( |$elem| $extract )?);
                            crate::info::FromListElemField::from_list_elem_field(raw_value)
                        }
                    ),*
                }
            }
        }
    };

    (
        @generateFieldExtraction,
        $elem:ident,
        custom,
        |$elem2:ident| $extract:expr
    ) => {
        {
            let $elem2 = & $elem;
            $extract
        }
    };

    (
        @generateFieldExtraction,
        $elem:ident,
        $field_key:literal,
        $field_type:ident
    ) => {
        $elem.$field_type(concat!($field_key, "\0"))
    }
}

trait FromListElemField<T> {
    fn from_list_elem_field(field: T) -> Self;
}

impl<T> FromListElemField<T> for T {
    fn from_list_elem_field(field: T) -> Self {
        field
    }
}

impl FromListElemField<Option<&str>> for String {
    fn from_list_elem_field(field: Option<&str>) -> Self {
        field
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| panic!("Unexpected null string in list"))
    }
}

impl FromListElemField<Option<&str>> for Option<String> {
    fn from_list_elem_field(field: Option<&str>) -> Self {
        field.map(ToOwned::to_owned)
    }
}

impl FromListElemField<Option<&str>> for Option<char> {
    fn from_list_elem_field(field: Option<&str>) -> Self {
        match field {
            Some(field) => match field.as_bytes() {
                &[] => None,
                &[single_byte] => Some(single_byte.into()),
                bytes => panic!(
                    "Expected 0 or 1 byte char in list, found {} bytes",
                    bytes.len()
                ),
            },
            None => panic!("Unexpected null string (char) in list"),
        }
    }
}

impl FromListElemField<i32> for u32 {
    fn from_list_elem_field(field: i32) -> Self {
        Self::try_from(field)
            .unwrap_or_else(|e| panic!("Unexpected negative integer in list: {}", e))
    }
}

impl FromListElemField<i32> for bool {
    fn from_list_elem_field(field: i32) -> Self {
        field != 0
    }
}

/// List types.
pub mod lists;
