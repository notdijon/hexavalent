use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw::c_int;

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
