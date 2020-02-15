use std::panic::{catch_unwind, UnwindSafe};

mod bindings;

// todo run under valgrind

pub fn catch_and_log_unwind<R>(f: impl FnOnce() -> R + UnwindSafe) -> Result<R, ()> {
    match catch_unwind(f) {
        Ok(x) => Ok(x),
        Err(e) => {
            let message = if let Some(s) = e.downcast_ref::<String>() {
                s.as_str()
            } else if let Some(s) = e.downcast_ref::<&'static str>() {
                s
            } else {
                &"<unknown>"
            };
            // todo check if this actually works
            eprintln!("Caught panic: {}", message);
            Err(())
        }
    }
}

// todo we should probably use the members of the `ph`, so filter out all the functions
#[rustfmt::skip]
pub use bindings::{
    // constants https://hexchat.readthedocs.io/en/latest/plugins.html#types-and-constants
    HEXCHAT_EAT_ALL,
    HEXCHAT_EAT_HEXCHAT,
    HEXCHAT_EAT_NONE,
    HEXCHAT_EAT_PLUGIN,

    HEXCHAT_FD_EXCEPTION,
    HEXCHAT_FD_NOTSOCKET,
    HEXCHAT_FD_READ,
    HEXCHAT_FD_WRITE,

    HEXCHAT_PRI_HIGH,
    HEXCHAT_PRI_HIGHEST,
    HEXCHAT_PRI_LOW,
    HEXCHAT_PRI_LOWEST,
    HEXCHAT_PRI_NORM,

    // types https://hexchat.readthedocs.io/en/latest/plugins.html#types-and-constants
    hexchat_plugin,
    hexchat_list,
    hexchat_hook,
    hexchat_context,
    hexchat_event_attrs,

    // general functions https://hexchat.readthedocs.io/en/latest/plugins.html#general-functions
    hexchat_command,
    hexchat_commandf,
    hexchat_print,
    hexchat_printf,
    hexchat_emit_print,
    hexchat_emit_print_attrs,
    hexchat_send_modes,
    hexchat_nickcmp,
    hexchat_strip,
    hexchat_free,
    hexchat_event_attrs_create,
    hexchat_event_attrs_free,

    // getting information https://hexchat.readthedocs.io/en/latest/plugins.html#getting-information
    hexchat_get_info,
    hexchat_get_prefs,
    hexchat_list_get,
    hexchat_list_fields,
    hexchat_list_next,
    hexchat_list_str,
    hexchat_list_int,
    hexchat_list_time,
    hexchat_list_free,

    // hook functions https://hexchat.readthedocs.io/en/latest/plugins.html#hook-functions
    hexchat_hook_command,
    hexchat_hook_fd,
    hexchat_hook_print,
    hexchat_hook_print_attrs,
    hexchat_hook_server,
    hexchat_hook_server_attrs,
    hexchat_hook_timer,
    hexchat_unhook,

    // context functions https://hexchat.readthedocs.io/en/latest/plugins.html#context-functions
    hexchat_find_context,
    hexchat_get_context,
    hexchat_set_context,

    // plugin preferences https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-preferences
    hexchat_pluginpref_set_str,
    hexchat_pluginpref_get_str,
    hexchat_pluginpref_set_int,
    hexchat_pluginpref_get_int,
    hexchat_pluginpref_delete,
    hexchat_pluginpref_list,

    // plugin gui https://hexchat.readthedocs.io/en/latest/plugins.html#plugin-gui
    hexchat_plugingui_add,
    hexchat_plugingui_remove,
};
