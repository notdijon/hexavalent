use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

mod bindings;

// constants https://hexchat.readthedocs.io/en/latest/plugins.html#types-and-constants
pub use bindings::{
    HEXCHAT_EAT_ALL, HEXCHAT_EAT_HEXCHAT, HEXCHAT_EAT_NONE, HEXCHAT_EAT_PLUGIN,
    HEXCHAT_FD_EXCEPTION, HEXCHAT_FD_NOTSOCKET, HEXCHAT_FD_READ, HEXCHAT_FD_WRITE,
    HEXCHAT_PRI_HIGH, HEXCHAT_PRI_HIGHEST, HEXCHAT_PRI_LOW, HEXCHAT_PRI_LOWEST, HEXCHAT_PRI_NORM,
};

// types https://hexchat.readthedocs.io/en/latest/plugins.html#types-and-constants
pub use bindings::{
    hexchat_context, hexchat_event_attrs, hexchat_hook, hexchat_list, hexchat_plugin,
};

// https://hexchat.readthedocs.io/en/latest/plugins.html#c.hexchat_emit_print
const SUCCESS: c_int = 1;
const FAILURE: c_int = 0;

pub fn int_to_result(ret_code: c_int) -> Result<(), ()> {
    match ret_code {
        SUCCESS => Ok(()),
        _ => Err(()),
    }
}

pub fn result_to_int(res: Result<(), ()>) -> c_int {
    match res {
        Ok(()) => SUCCESS,
        Err(_) => FAILURE,
    }
}

pub trait StrExt {
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

/// Holds a HexChat `word` or `word_eol` pointer.
pub struct WordPtr {
    /// Always points to a valid `word` or `word_eol` array.
    ptr: *mut *mut c_char,
}

impl WordPtr {
    /// Creates a new `WordPtr` from a `word` or `word_eol` pointer.
    ///
    /// # Safety
    ///
    /// `word` must be a `word` or `word_eol` pointer from HexChat.
    /// Calling this function with pointers from anywhere else, even other pointers returned from HexChat,
    /// is undefined behavior.
    ///
    /// See: https://hexchat.readthedocs.io/en/latest/plugins.html#what-s-word-and-word-eol
    ///
    /// It is your responsibility to ensure that the returned `WordPtr` does not outlive the `word` pointer used to create it.
    pub unsafe fn new(word: *mut *mut c_char) -> Self {
        Self { ptr: word }
    }
}

/// Converts `word` or `word_eol` to a `&CStr` slice.
///
/// # Panics
///
/// If any element of `word` contains invalid UTF8.
pub fn with_parsed_words<R>(word: WordPtr, f: impl FnOnce(&[&str; 32]) -> R) -> R {
    let word = word.ptr;

    // https://hexchat.readthedocs.io/en/latest/plugins.html#what-s-word-and-word-eol
    // Safety: first index is reserved, per documentation
    let word = unsafe { word.offset(1) };

    let mut words = [""; 32];
    for i in 0..words.len() {
        // Safety: word points to a valid null-terminated array, so we cannot read past the end or wrap
        let elem = unsafe { *word.offset(i as isize) };
        if elem.is_null() {
            break;
        }
        // Safety: word points to valid strings; words does not outlive this function
        let cstr = unsafe { CStr::from_ptr(elem) };
        words[i] = cstr
            .to_str()
            .unwrap_or_else(|e| panic!("Invalid UTF8 in field index {}: {}", i, e));
    }

    // hexchat always passes in 32 args, so just give them all of it
    // not by-value, because that results in a stack-to-stack memcpy, even when everything is inlined :(
    f(&words)
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
        assert_matches!(Cow::Owned(_), owner);
        assert_eq!(owner.as_ref(), cs("hello\0"));

        let owner = "hello\0".into_cstr();
        assert_matches!(Cow::Borrowed(_), owner);
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
