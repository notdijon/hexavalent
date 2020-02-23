use std::cell::{Cell, RefCell};
use std::collections::HashSet;

use hexavalent::hook::{Eat, Priority};
use hexavalent::print::events::ChannelMessage;
use hexavalent::print::PrintEvent;
use hexavalent::{export_plugin, Plugin, PluginHandle};

#[derive(Default)]
struct SimplePlugin {
    count: Cell<u64>,
    nicks: RefCell<HashSet<String>>,
}

impl SimplePlugin {
    fn message_cb(
        &self,
        _ph: PluginHandle<'_, Self>,
        [nick, _text, _mode, _ident]: <ChannelMessage as PrintEvent<'_>>::Args,
    ) -> Eat {
        self.count.set(self.count.get() + 1);
        self.nicks.borrow_mut().insert(nick.to_string());
        Eat::None
    }
}

impl Plugin for SimplePlugin {
    fn init(&self, ph: PluginHandle<'_, Self>) {
        ph.hook_print(ChannelMessage, Priority::Normal, Self::message_cb);
        ph.hook_command(
            "count\0",
            "Usage: COUNT, print message count\0",
            Priority::Normal,
            |plugin, ph, _words| {
                let count = plugin.count.get();
                let nicks = plugin.nicks.borrow().len();
                ph.print(&format!(
                    "Received {} messages from {} unique nicks.\0",
                    count, nicks
                ));
                Eat::All
            },
        );
        ph.print("Plugin loaded successfully!\0");
    }

    fn deinit(&self, ph: PluginHandle<'_, Self>) {
        ph.print("Unloading plugin...\0");
    }
}

export_plugin!(
    SimplePlugin,
    "Simple Example",
    "Just counts messages, try /count",
    "1.0.0"
);
