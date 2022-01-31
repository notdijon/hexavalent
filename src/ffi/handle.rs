use std::ffi::c_void;
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int};
use std::ptr::NonNull;

use libc::time_t;

use crate::ffi::{
    hexchat_context, hexchat_event_attrs, hexchat_hook, hexchat_list, hexchat_plugin,
};

#[derive(Debug, Copy, Clone)]
pub(crate) struct RawPluginHandle<'ph> {
    /// Always points to an instance of `hexchat_plugin` valid for `'ph`.
    handle: NonNull<hexchat_plugin>,
    _lifetime: PhantomData<&'ph hexchat_plugin>,
}

impl<'ph> RawPluginHandle<'ph> {
    /// Creates a new `RawPluginHandle` from a native `hexchat_plugin`.
    ///
    /// # Safety
    ///
    /// `handle` must point an instance of `hexchat_plugin` valid for the entire lifetime `'ph`.
    pub(crate) unsafe fn new(handle: NonNull<hexchat_plugin>) -> Self {
        Self {
            handle,
            _lifetime: PhantomData,
        }
    }
}

impl RawPluginHandle<'_> {
    pub(crate) unsafe fn hexchat_hook_command(
        self,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            user_data: *mut c_void,
        ) -> c_int,
        help_text: *const c_char,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_hook_command)(
                self.handle.as_ptr(),
                name,
                pri,
                callback,
                help_text,
                userdata,
            )
        }
    }

    pub(crate) unsafe fn hexchat_hook_server(
        self,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_hook_server)(
                self.handle.as_ptr(),
                name,
                pri,
                callback,
                userdata,
            )
        }
    }

    pub(crate) unsafe fn hexchat_hook_print(
        self,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern "C" fn(word: *mut *mut c_char, user_data: *mut c_void) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_hook_print)(
                self.handle.as_ptr(),
                name,
                pri,
                callback,
                userdata,
            )
        }
    }

    pub(crate) unsafe fn hexchat_hook_timer(
        self,
        timeout: c_int,
        callback: unsafe extern "C" fn(user_data: *mut c_void) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_hook_timer)(
                self.handle.as_ptr(),
                timeout,
                callback,
                userdata,
            )
        }
    }

    pub(crate) unsafe fn hexchat_unhook(self, hook: *mut hexchat_hook) -> *mut c_void {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_unhook)(self.handle.as_ptr(), hook) }
    }

    pub(crate) unsafe fn hexchat_print(self, text: *const c_char) {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_print)(self.handle.as_ptr(), text) }
    }

    pub(crate) unsafe fn hexchat_command(self, command: *const c_char) {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_command)(self.handle.as_ptr(), command) }
    }

    pub(crate) unsafe fn hexchat_nickcmp(self, s1: *const c_char, s2: *const c_char) -> c_int {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_nickcmp)(self.handle.as_ptr(), s1, s2) }
    }

    pub(crate) unsafe fn hexchat_set_context(self, ctx: *mut hexchat_context) -> c_int {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_set_context)(self.handle.as_ptr(), ctx) }
    }

    pub(crate) unsafe fn hexchat_find_context(
        self,
        servname: *const c_char,
        channel: *const c_char,
    ) -> *mut hexchat_context {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_find_context)(self.handle.as_ptr(), servname, channel)
        }
    }

    pub(crate) unsafe fn hexchat_get_context(self) -> *mut hexchat_context {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_get_context)(self.handle.as_ptr()) }
    }

    pub(crate) unsafe fn hexchat_get_info(self, id: *const c_char) -> *const c_char {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_get_info)(self.handle.as_ptr(), id) }
    }

    pub(crate) unsafe fn hexchat_get_prefs(
        self,
        name: *const c_char,
        string: *mut *const c_char,
        integer: *mut c_int,
    ) -> c_int {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_get_prefs)(self.handle.as_ptr(), name, string, integer)
        }
    }

    pub(crate) unsafe fn hexchat_list_get(self, name: *const c_char) -> *mut hexchat_list {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_list_get)(self.handle.as_ptr(), name) }
    }

    pub(crate) unsafe fn hexchat_list_free(self, xlist: *mut hexchat_list) {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_list_free)(self.handle.as_ptr(), xlist) }
    }

    pub(crate) unsafe fn hexchat_list_next(self, xlist: *mut hexchat_list) -> c_int {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_list_next)(self.handle.as_ptr(), xlist) }
    }

    pub(crate) unsafe fn hexchat_list_str(
        self,
        xlist: *mut hexchat_list,
        name: *const c_char,
    ) -> *const c_char {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_list_str)(self.handle.as_ptr(), xlist, name) }
    }

    pub(crate) unsafe fn hexchat_list_int(
        self,
        xlist: *mut hexchat_list,
        name: *const c_char,
    ) -> c_int {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_list_int)(self.handle.as_ptr(), xlist, name) }
    }

    pub(crate) unsafe fn hexchat_plugingui_add(
        self,
        filename: *const c_char,
        name: *const c_char,
        desc: *const c_char,
        version: *const c_char,
        reserved: *mut c_char,
    ) -> *mut c_void {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_plugingui_add)(
                self.handle.as_ptr(),
                filename,
                name,
                desc,
                version,
                reserved,
            )
        }
    }

    pub(crate) unsafe fn hexchat_plugingui_remove(self, handle: *mut c_void) {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_plugingui_remove)(self.handle.as_ptr(), handle) }
    }

    pub(crate) unsafe fn hexchat_emit_print(
        self,
        event_name: *const c_char,
        a1: *const c_char,
        a2: *const c_char,
        a3: *const c_char,
        a4: *const c_char,
        a5: *const c_char,
    ) -> c_int {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_emit_print)(
                self.handle.as_ptr(),
                event_name,
                a1,
                a2,
                a3,
                a4,
                a5,
            )
        }
    }

    pub(crate) unsafe fn hexchat_list_time(
        self,
        xlist: *mut hexchat_list,
        name: *const c_char,
    ) -> time_t {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_list_time)(self.handle.as_ptr(), xlist, name) }
    }

    pub(crate) unsafe fn hexchat_send_modes(
        self,
        targets: *mut *const c_char,
        ntargets: c_int,
        modes_per_line: c_int,
        sign: c_char,
        mode: c_char,
    ) {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_send_modes)(
                self.handle.as_ptr(),
                targets,
                ntargets,
                modes_per_line,
                sign,
                mode,
            )
        }
    }

    pub(crate) unsafe fn hexchat_strip(
        self,
        str: *const c_char,
        len: c_int,
        flags: c_int,
    ) -> *mut c_char {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_strip)(self.handle.as_ptr(), str, len, flags) }
    }

    pub(crate) unsafe fn hexchat_free(self, ptr: *mut c_void) {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_free)(self.handle.as_ptr(), ptr) }
    }

    pub(crate) unsafe fn hexchat_pluginpref_set_str(
        self,
        var: *const c_char,
        value: *const c_char,
    ) -> c_int {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_pluginpref_set_str)(self.handle.as_ptr(), var, value)
        }
    }

    pub(crate) unsafe fn hexchat_pluginpref_get_str(
        self,
        var: *const c_char,
        dest: *mut c_char,
    ) -> c_int {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_pluginpref_get_str)(self.handle.as_ptr(), var, dest)
        }
    }

    pub(crate) unsafe fn hexchat_pluginpref_set_int(
        self,
        var: *const c_char,
        value: c_int,
    ) -> c_int {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_pluginpref_set_int)(self.handle.as_ptr(), var, value)
        }
    }

    pub(crate) unsafe fn hexchat_pluginpref_get_int(self, var: *const c_char) -> c_int {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_pluginpref_get_int)(self.handle.as_ptr(), var) }
    }

    pub(crate) unsafe fn hexchat_pluginpref_delete(self, var: *const c_char) -> c_int {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_pluginpref_delete)(self.handle.as_ptr(), var) }
    }

    pub(crate) unsafe fn hexchat_pluginpref_list(self, dest: *mut c_char) -> c_int {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_pluginpref_list)(self.handle.as_ptr(), dest) }
    }

    pub(crate) unsafe fn hexchat_hook_server_attrs(
        self,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut c_char,
            word_eol: *mut *mut c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_hook_server_attrs)(
                self.handle.as_ptr(),
                name,
                pri,
                callback,
                userdata,
            )
        }
    }

    pub(crate) unsafe fn hexchat_hook_print_attrs(
        self,
        name: *const c_char,
        pri: c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut c_void,
        ) -> c_int,
        userdata: *mut c_void,
    ) -> *mut hexchat_hook {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_hook_print_attrs)(
                self.handle.as_ptr(),
                name,
                pri,
                callback,
                userdata,
            )
        }
    }

    pub(crate) unsafe fn hexchat_emit_print_attrs(
        self,
        attrs: *mut hexchat_event_attrs,
        event_name: *const c_char,
        a1: *const c_char,
        a2: *const c_char,
        a3: *const c_char,
        a4: *const c_char,
        a5: *const c_char,
    ) -> c_int {
        // Safety: forwarded to caller
        unsafe {
            ((*self.handle.as_ptr()).hexchat_emit_print_attrs)(
                self.handle.as_ptr(),
                attrs,
                event_name,
                a1,
                a2,
                a3,
                a4,
                a5,
            )
        }
    }

    pub(crate) unsafe fn hexchat_event_attrs_create(self) -> *mut hexchat_event_attrs {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_event_attrs_create)(self.handle.as_ptr()) }
    }

    pub(crate) unsafe fn hexchat_event_attrs_free(self, attrs: *mut hexchat_event_attrs) {
        // Safety: forwarded to caller
        unsafe { ((*self.handle.as_ptr()).hexchat_event_attrs_free)(self.handle.as_ptr(), attrs) }
    }
}
