//! Info lists.

use std::convert::TryFrom;
use std::ops::Deref;
use std::str::Split;

use crate::str::{HexStr, HexString};

/// A list that can be retrieved from HexChat.
///
/// Used with [`PluginHandle::get_list`](crate::PluginHandle::get_list).
///
/// This trait is sealed and cannot be implemented outside of `hexavalent`.
pub trait List: private::ListImpl + 'static
where
    Self::Elem: private::FromListElem,
{
    /// The type of elements of the list.
    // todo with GATs, it _might_ be nice to have Elem/BorrowedElem<'a>, so that we can avoid allocation
    //  (but we'd probably have to make get_list_with unsafe due to invalidation of the string)
    type Elem: 'static;
}

pub(crate) mod private {
    use crate::ffi::ListElem;
    use std::ffi::CStr;

    pub trait ListImpl {
        const NAME: &'static CStr;
    }

    #[allow(unreachable_pub)]
    pub trait FromListElem: Sized {
        fn from_list_elem(elem: ListElem<'_>) -> Self;
    }
}

macro_rules! list {
    (
        $struct_name:ident,
        $list_name:literal,
        $description:literal,
        $elem_desc:literal,
        $elem_ty:ident {
            $(
                [ $( $field_key:literal )? $( $custom:ident )?, $field_desc:literal, $( $field_type:ident )? $( |$elem:ident| $extract:expr )? ]
                $rust_field_name:ident : $rust_field_type:ty => $rust_method_type:ty
            ),* $(,)?
        }
    ) => {
        #[doc = "`"]
        #[doc = $list_name]
        #[doc = "`"]
        #[doc = ""]
        #[doc = $description]
        #[derive(Debug, Copy, Clone)]
        pub struct $struct_name;

        impl crate::list::private::ListImpl for $struct_name {
            const NAME: &'static ::std::ffi::CStr = match ::std::ffi::CStr::from_bytes_with_nul(concat!($list_name, "\0").as_bytes()) {
                Ok(name) => name,
                Err(_) => unreachable!(),
            };
        }

        impl crate::list::List for $struct_name {
            type Elem = $elem_ty;
        }

        #[doc = $elem_desc]
        ///
        /// See the [`List`](crate::list::List) trait for usage.
        #[derive(Debug, Clone)]
        pub struct $elem_ty {
            $(
                $rust_field_name: $rust_field_type,
            )*
        }

        impl $elem_ty {
            $(
                #[doc = $field_desc]
                pub fn $rust_field_name(&self) -> $rust_method_type {
                    crate::list::ProjectListElemField::project_list_elem_field(&self.$rust_field_name)
                }
            )*
        }

        impl crate::list::private::FromListElem for $elem_ty {
            fn from_list_elem(elem: crate::ffi::ListElem<'_>) -> Self {
                Self {
                    $(
                        $rust_field_name: {
                            let raw_value = list!(@generateFieldExtraction, elem, $( $field_key )? $( $custom )?, $( $field_type )? $( |$elem| $extract )?);
                            crate::list::FromListElemField::from_list_elem_field(raw_value)
                        },
                    )*
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
        {
            const NAME: &::std::ffi::CStr = match ::std::ffi::CStr::from_bytes_with_nul(concat!($field_key, "\0").as_bytes()) {
                Ok(name) => name,
                Err(_) => unreachable!(),
            };
            $elem.$field_type(NAME)
        }
    }
}

#[derive(Debug, Clone)]
struct SplitByCommas(String);

trait FromListElemField<T> {
    fn from_list_elem_field(field: T) -> Self;
}

impl<T> FromListElemField<T> for T {
    fn from_list_elem_field(field: T) -> Self {
        field
    }
}

impl FromListElemField<Option<&HexStr>> for HexString {
    fn from_list_elem_field(field: Option<&HexStr>) -> Self {
        field
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| panic!("Unexpected null string in list"))
    }
}

impl FromListElemField<Option<&HexStr>> for Option<HexString> {
    fn from_list_elem_field(field: Option<&HexStr>) -> Self {
        field.map(ToOwned::to_owned)
    }
}

impl FromListElemField<Option<&HexStr>> for Option<char> {
    fn from_list_elem_field(field: Option<&HexStr>) -> Self {
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

impl FromListElemField<Option<&HexStr>> for SplitByCommas {
    fn from_list_elem_field(field: Option<&HexStr>) -> SplitByCommas {
        SplitByCommas(field.map(|s| s.deref().to_owned()).unwrap_or_default())
    }
}

trait ProjectListElemField<'a, T> {
    fn project_list_elem_field(&'a self) -> T;
}

impl<'a, T: Copy> ProjectListElemField<'a, T> for T {
    fn project_list_elem_field(&self) -> T {
        *self
    }
}

impl<'a> ProjectListElemField<'a, &'a HexStr> for HexString {
    fn project_list_elem_field(&self) -> &HexStr {
        self
    }
}

impl<'a> ProjectListElemField<'a, Option<&'a HexStr>> for Option<HexString> {
    fn project_list_elem_field(&self) -> Option<&HexStr> {
        self.as_deref()
    }
}

impl<'a> ProjectListElemField<'a, Split<'a, char>> for SplitByCommas {
    fn project_list_elem_field(&'a self) -> Split<'a, char> {
        self.0.split(',')
    }
}

mod impls;

pub use impls::*;
