use std::panic::{catch_unwind, UnwindSafe};

use crate::PluginHandle;

mod bindings;

pub fn catch_and_log_unwind<R>(
    ph: PluginHandle<'_>,
    ctxt_msg: &str,
    f: impl FnOnce() -> R + UnwindSafe,
) -> Result<R, ()> {
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
            ph.print(format!(
                "WARNING: `hexavalent` caught panic (in `{}`): {}\0",
                ctxt_msg, message
            ));
            Err(())
        }
    }
}

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
