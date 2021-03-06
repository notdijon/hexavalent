/* automatically generated by rust-bindgen */

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(missing_docs)]
use libc::time_t;

pub const HEXCHAT_PRI_HIGHEST: u32 = 127;
pub const HEXCHAT_PRI_HIGH: u32 = 64;
pub const HEXCHAT_PRI_NORM: u32 = 0;
pub const HEXCHAT_PRI_LOW: i32 = -64;
pub const HEXCHAT_PRI_LOWEST: i32 = -128;
pub const HEXCHAT_FD_READ: u32 = 1;
pub const HEXCHAT_FD_WRITE: u32 = 2;
pub const HEXCHAT_FD_EXCEPTION: u32 = 4;
pub const HEXCHAT_FD_NOTSOCKET: u32 = 8;
pub const HEXCHAT_EAT_NONE: u32 = 0;
pub const HEXCHAT_EAT_HEXCHAT: u32 = 1;
pub const HEXCHAT_EAT_PLUGIN: u32 = 2;
pub const HEXCHAT_EAT_ALL: u32 = 3;
pub type __time_t = ::std::os::raw::c_long;
pub type hexchat_plugin = _hexchat_plugin;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hexchat_list {
    _unused: [u8; 0],
}
pub type hexchat_list = _hexchat_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hexchat_hook {
    _unused: [u8; 0],
}
pub type hexchat_hook = _hexchat_hook;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hexchat_context {
    _unused: [u8; 0],
}
pub type hexchat_context = _hexchat_context;
#[repr(C)]
pub struct hexchat_event_attrs {
    pub server_time_utc: time_t,
    #[cfg(feature = "__unstable_ircv3_line_in_event_attrs")]
    pub ircv3_line: *const ::std::os::raw::c_char,
}
#[test]
#[cfg(not(feature = "__unstable_ircv3_line_in_event_attrs"))]
fn bindgen_test_layout_hexchat_event_attrs() {
    assert_eq!(
        ::std::mem::size_of::<hexchat_event_attrs>(),
        8usize,
        concat!("Size of: ", stringify!(hexchat_event_attrs))
    );
    assert_eq!(
        ::std::mem::align_of::<hexchat_event_attrs>(),
        8usize,
        concat!("Alignment of ", stringify!(hexchat_event_attrs))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<hexchat_event_attrs>())).server_time_utc as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(hexchat_event_attrs),
            "::",
            stringify!(server_time_utc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _hexchat_plugin {
    pub hexchat_hook_command: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
        pri: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut ::std::os::raw::c_char,
            word_eol: *mut *mut ::std::os::raw::c_char,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        help_text: *const ::std::os::raw::c_char,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut hexchat_hook,

    pub hexchat_hook_server: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
        pri: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut ::std::os::raw::c_char,
            word_eol: *mut *mut ::std::os::raw::c_char,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut hexchat_hook,

    pub hexchat_hook_print: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
        pri: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut ::std::os::raw::c_char,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut hexchat_hook,

    pub hexchat_hook_timer: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        timeout: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut hexchat_hook,

    pub hexchat_hook_fd: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        fd: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            fd: ::std::os::raw::c_int,
            flags: ::std::os::raw::c_int,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut hexchat_hook,

    pub hexchat_unhook: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        hook: *mut hexchat_hook,
    ) -> *mut ::std::os::raw::c_void,

    pub hexchat_print:
        unsafe extern "C" fn(ph: *mut hexchat_plugin, text: *const ::std::os::raw::c_char),

    pub hexchat_printf:
        unsafe extern "C" fn(ph: *mut hexchat_plugin, format: *const ::std::os::raw::c_char, ...),

    pub hexchat_command:
        unsafe extern "C" fn(ph: *mut hexchat_plugin, command: *const ::std::os::raw::c_char),

    pub hexchat_commandf:
        unsafe extern "C" fn(ph: *mut hexchat_plugin, format: *const ::std::os::raw::c_char, ...),

    pub hexchat_nickcmp: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        s1: *const ::std::os::raw::c_char,
        s2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,

    pub hexchat_set_context: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        ctx: *mut hexchat_context,
    ) -> ::std::os::raw::c_int,

    pub hexchat_find_context: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        servname: *const ::std::os::raw::c_char,
        channel: *const ::std::os::raw::c_char,
    ) -> *mut hexchat_context,

    pub hexchat_get_context: unsafe extern "C" fn(ph: *mut hexchat_plugin) -> *mut hexchat_context,

    pub hexchat_get_info: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        id: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,

    pub hexchat_get_prefs: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
        string: *mut *const ::std::os::raw::c_char,
        integer: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,

    pub hexchat_list_get: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
    ) -> *mut hexchat_list,

    pub hexchat_list_free: unsafe extern "C" fn(ph: *mut hexchat_plugin, xlist: *mut hexchat_list),

    pub hexchat_list_fields: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
    ) -> *const *const ::std::os::raw::c_char,

    pub hexchat_list_next: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
    ) -> ::std::os::raw::c_int,

    pub hexchat_list_str: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
        name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char,

    pub hexchat_list_int: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
        name: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,

    pub hexchat_plugingui_add: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        filename: *const ::std::os::raw::c_char,
        name: *const ::std::os::raw::c_char,
        desc: *const ::std::os::raw::c_char,
        version: *const ::std::os::raw::c_char,
        reserved: *mut ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_void,

    pub hexchat_plugingui_remove:
        unsafe extern "C" fn(ph: *mut hexchat_plugin, handle: *mut ::std::os::raw::c_void),

    pub hexchat_emit_print: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        event_name: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int,

    pub hexchat_read_fd: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        src: *mut ::std::os::raw::c_void,
        buf: *mut ::std::os::raw::c_char,
        len: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,

    pub hexchat_list_time: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        xlist: *mut hexchat_list,
        name: *const ::std::os::raw::c_char,
    ) -> time_t,

    pub hexchat_gettext: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        msgid: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char,

    pub hexchat_send_modes: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        targets: *mut *const ::std::os::raw::c_char,
        ntargets: ::std::os::raw::c_int,
        modes_per_line: ::std::os::raw::c_int,
        sign: ::std::os::raw::c_char,
        mode: ::std::os::raw::c_char,
    ),

    pub hexchat_strip: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        str: *const ::std::os::raw::c_char,
        len: ::std::os::raw::c_int,
        flags: ::std::os::raw::c_int,
    ) -> *mut ::std::os::raw::c_char,

    pub hexchat_free:
        unsafe extern "C" fn(ph: *mut hexchat_plugin, ptr: *mut ::std::os::raw::c_void),

    pub hexchat_pluginpref_set_str: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        var: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,

    pub hexchat_pluginpref_get_str: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        var: *const ::std::os::raw::c_char,
        dest: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,

    pub hexchat_pluginpref_set_int: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        var: *const ::std::os::raw::c_char,
        value: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int,

    pub hexchat_pluginpref_get_int: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        var: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,

    pub hexchat_pluginpref_delete: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        var: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,

    pub hexchat_pluginpref_list: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        dest: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int,

    pub hexchat_hook_server_attrs: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
        pri: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut ::std::os::raw::c_char,
            word_eol: *mut *mut ::std::os::raw::c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut hexchat_hook,

    pub hexchat_hook_print_attrs: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        name: *const ::std::os::raw::c_char,
        pri: ::std::os::raw::c_int,
        callback: unsafe extern "C" fn(
            word: *mut *mut ::std::os::raw::c_char,
            attrs: *mut hexchat_event_attrs,
            user_data: *mut ::std::os::raw::c_void,
        ) -> ::std::os::raw::c_int,
        userdata: *mut ::std::os::raw::c_void,
    ) -> *mut hexchat_hook,

    pub hexchat_emit_print_attrs: unsafe extern "C" fn(
        ph: *mut hexchat_plugin,
        attrs: *mut hexchat_event_attrs,
        event_name: *const ::std::os::raw::c_char,
        ...
    ) -> ::std::os::raw::c_int,

    pub hexchat_event_attrs_create:
        unsafe extern "C" fn(ph: *mut hexchat_plugin) -> *mut hexchat_event_attrs,

    pub hexchat_event_attrs_free:
        unsafe extern "C" fn(ph: *mut hexchat_plugin, attrs: *mut hexchat_event_attrs),
}
#[test]
fn bindgen_test_layout__hexchat_plugin() {
    assert_eq!(
        ::std::mem::size_of::<_hexchat_plugin>(),
        336usize,
        concat!("Size of: ", stringify!(_hexchat_plugin))
    );
    assert_eq!(
        ::std::mem::align_of::<_hexchat_plugin>(),
        8usize,
        concat!("Alignment of ", stringify!(_hexchat_plugin))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_hook_command as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_hook_command)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_hook_server as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_hook_server)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_hook_print as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_hook_print)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_hook_timer as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_hook_timer)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_hook_fd as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_hook_fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_unhook as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_unhook)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_print as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_print)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_printf as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_printf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_command as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_command)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_commandf as *const _ as usize
        },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_commandf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_nickcmp as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_nickcmp)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_set_context as *const _ as usize
        },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_set_context)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_find_context as *const _ as usize
        },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_find_context)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_get_context as *const _ as usize
        },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_get_context)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_get_info as *const _ as usize
        },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_get_info)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_get_prefs as *const _ as usize
        },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_get_prefs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_list_get as *const _ as usize
        },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_list_get)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_list_free as *const _ as usize
        },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_list_free)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_list_fields as *const _ as usize
        },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_list_fields)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_list_next as *const _ as usize
        },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_list_next)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_list_str as *const _ as usize
        },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_list_str)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_list_int as *const _ as usize
        },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_list_int)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_plugingui_add as *const _ as usize
        },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_plugingui_add)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_plugingui_remove as *const _
                as usize
        },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_plugingui_remove)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_emit_print as *const _ as usize
        },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_emit_print)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_read_fd as *const _ as usize },
        200usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_read_fd)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_list_time as *const _ as usize
        },
        208usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_list_time)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_gettext as *const _ as usize },
        216usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_gettext)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_send_modes as *const _ as usize
        },
        224usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_send_modes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_strip as *const _ as usize },
        232usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_strip)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_free as *const _ as usize },
        240usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_free)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_pluginpref_set_str as *const _
                as usize
        },
        248usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_pluginpref_set_str)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_pluginpref_get_str as *const _
                as usize
        },
        256usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_pluginpref_get_str)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_pluginpref_set_int as *const _
                as usize
        },
        264usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_pluginpref_set_int)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_pluginpref_get_int as *const _
                as usize
        },
        272usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_pluginpref_get_int)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_pluginpref_delete as *const _
                as usize
        },
        280usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_pluginpref_delete)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_pluginpref_list as *const _ as usize
        },
        288usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_pluginpref_list)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_hook_server_attrs as *const _
                as usize
        },
        296usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_hook_server_attrs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_hook_print_attrs as *const _
                as usize
        },
        304usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_hook_print_attrs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_emit_print_attrs as *const _
                as usize
        },
        312usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_emit_print_attrs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_event_attrs_create as *const _
                as usize
        },
        320usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_event_attrs_create)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<_hexchat_plugin>())).hexchat_event_attrs_free as *const _
                as usize
        },
        328usize,
        concat!(
            "Offset of field: ",
            stringify!(_hexchat_plugin),
            "::",
            stringify!(hexchat_event_attrs_free)
        )
    );
}
