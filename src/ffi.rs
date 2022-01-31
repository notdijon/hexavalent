use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};
use std::ptr::NonNull;

use time::OffsetDateTime;

#[allow(missing_debug_implementations, missing_docs, unreachable_pub)]
mod binding;

mod handle;

// constants https://hexchat.readthedocs.io/en/latest/plugins.html#types-and-constants
pub(crate) use binding::{
    HEXCHAT_EAT_ALL, HEXCHAT_EAT_HEXCHAT, HEXCHAT_EAT_NONE, HEXCHAT_EAT_PLUGIN, HEXCHAT_PRI_HIGH,
    HEXCHAT_PRI_HIGHEST, HEXCHAT_PRI_LOW, HEXCHAT_PRI_LOWEST, HEXCHAT_PRI_NORM,
};

// types https://hexchat.readthedocs.io/en/latest/plugins.html#types-and-constants
pub(crate) use binding::{hexchat_context, hexchat_event_attrs, hexchat_hook, hexchat_list};
// this is used publicly by generated code
pub use binding::hexchat_plugin;

pub(crate) use handle::RawPluginHandle;

// https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print
const SUCCESS: c_int = 1;
const FAILURE: c_int = 0;

pub(crate) fn int_to_result(ret_code: c_int) -> Result<(), ()> {
    match ret_code {
        SUCCESS => Ok(()),
        _ => Err(()),
    }
}

pub(crate) fn result_to_int(res: Result<(), ()>) -> c_int {
    match res {
        Ok(()) => SUCCESS,
        Err(_) => FAILURE,
    }
}

pub(crate) trait StrExt {
    type CSTR: AsRef<CStr>;

    fn into_cstr(self) -> Self::CSTR;
}

impl<'a> StrExt for &'a str {
    type CSTR = Cow<'a, CStr>;

    fn into_cstr(self) -> Self::CSTR {
        // check last byte up front to avoid scanning the string twice if it does not end with null
        if self.as_bytes().last().copied() == Some(0) {
            Cow::Borrowed(CStr::from_bytes_with_nul(self.as_bytes()).unwrap())
        } else {
            Cow::Owned(CString::new(self).unwrap())
        }
    }
}

/// Converts `word` or `word_eol` to an iterator over `&CStr`.
///
/// # Safety
///
/// `word` must be a `word` or `word_eol` pointer from HexChat.
///
/// `word` must be valid for the entire lifetime `'a`.
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) unsafe fn word_to_iter<'a>(
    word: &'a *mut *mut c_char,
) -> impl Iterator<Item = &'a CStr> {
    // make it obvious that this is a non-raw-pointer deref
    let word: *mut *mut c_char = *word;

    // https://hexchat.readthedocs.io/en/latest/plugins.html#what-s-word-and-word-eol
    // Safety: first index is reserved, per documentation
    let word = unsafe { word.add(1) };

    struct WordIter<'a> {
        word: *mut *mut c_char,
        _lifetime: PhantomData<&'a *mut c_char>,
    }

    impl<'a> Iterator for WordIter<'a> {
        type Item = &'a CStr;

        fn next(&mut self) -> Option<Self::Item> {
            // Safety: word points to a valid null-terminated array, so we cannot read past the end or wrap
            let elem = unsafe { *self.word };
            if elem.is_null() {
                None
            } else {
                // Safety: elem is not null, so there is at least one more element in the array (possibly null)
                self.word = unsafe { self.word.add(1) };
                // Safety: word points to valid strings; words does not outlive 'a
                Some(unsafe { CStr::from_ptr::<'a>(elem) })
            }
        }

        fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
            while n > 0 {
                let elem = unsafe { *self.word };
                if elem.is_null() {
                    break;
                } else {
                    // Safety: elem is not null, so there is at least one more element in the array (possibly null)
                    self.word = unsafe { self.word.add(1) };
                }
                n -= 1;
            }

            self.next()
        }
    }

    WordIter::<'a> {
        word,
        _lifetime: PhantomData,
    }
}

#[allow(unreachable_pub)]
#[derive(Debug)]
pub struct ListElem<'a> {
    raw: RawPluginHandle<'a>,
    /// Always points to a valid list element.
    list_ptr: NonNull<hexchat_list>,
}

impl<'a> ListElem<'a> {
    /// Creates a safe wrapper around a list element.
    ///
    /// # Safety
    ///
    /// `list` must point to a `hexchat_list` element (e.g. one for which `hexchat_list_next` returned true),
    /// which is valid for the entire lifetime `'a`.
    ///
    /// You must not interact with Hexchat in any way that could invalidate this list elem while it exists.
    /// Notably, this includes calling `hexchat_list_next` on the same list to get another element,
    /// but may also include other operations (e.g. switching channels). To be safe, do not call
    /// any Hexchat functions while a `ListElem` exists.
    pub(crate) unsafe fn new(raw: RawPluginHandle<'a>, list_ptr: NonNull<hexchat_list>) -> Self {
        Self { raw, list_ptr }
    }

    pub(crate) fn string<'elem>(&'elem self, null_terminated_name: &str) -> Option<&'elem str> {
        assert!(null_terminated_name.as_bytes().last().copied() == Some(0));
        let name = null_terminated_name.as_ptr().cast();

        // Safety: list_ptr is valid per ListElem precondition, name is a null-terminated string
        let ptr = unsafe { self.raw.hexchat_list_str(self.list_ptr.as_ptr(), name) };

        if ptr.is_null() {
            return None;
        }

        // Safety: hexchat_list_str gets a valid string or null, temporary does not outlive the list elem
        let str = unsafe { CStr::from_ptr(ptr) }
            .to_str()
            .unwrap_or_else(|e| panic!("Invalid UTF8 from `hexchat_get_prefs`: {}", e));

        Some(str)
    }

    pub(crate) fn int(&self, null_terminated_name: &str) -> i32 {
        assert!(null_terminated_name.as_bytes().last().copied() == Some(0));
        let name = null_terminated_name.as_ptr().cast();

        // Safety: list_ptr is valid per ListElem precondition, name is a null-terminated string
        unsafe { self.raw.hexchat_list_int(self.list_ptr.as_ptr(), name) }
    }

    pub(crate) fn time(&self, null_terminated_name: &str) -> OffsetDateTime {
        assert!(null_terminated_name.as_bytes().last().copied() == Some(0));
        let name = null_terminated_name.as_ptr().cast();

        // Safety: list_ptr is valid per ListElem precondition, name is a null-terminated string
        let time = unsafe { self.raw.hexchat_list_time(self.list_ptr.as_ptr(), name) };

        OffsetDateTime::from_unix_timestamp(time)
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Cow;

    use super::*;

    fn cs(s: &str) -> &CStr {
        CStr::from_bytes_with_nul(s.as_bytes()).unwrap()
    }

    #[test]
    fn intocstr_str() {
        let owner = "hello".into_cstr();
        assert!(matches!(owner, Cow::Owned(_)));
        assert_eq!(owner.as_ref(), cs("hello\0"));

        let owner = "hello\0".into_cstr();
        assert!(matches!(owner, Cow::Borrowed(_)));
        assert_eq!(owner.as_ref(), cs("hello\0"));
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
