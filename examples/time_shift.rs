use std::cell::Cell;
use std::time::Duration;

use hexavalent::event::print::{ChannelMessage, MessageSend, PrintEvent, PrivateMessage};
use hexavalent::event::EventAttrs;
use hexavalent::hook::{Eat, Priority};
use hexavalent::{export_plugin, Plugin, PluginHandle};

#[derive(Default)]
struct TimeShiftPlugin {
    /// True if we're currently inside a hook. Prevents infinite recursion when we emit an event.
    inside_hook: Cell<bool>,
    /// Number of seconds by which to offset all timestamps.
    offset_seconds: Cell<i64>,
}

impl TimeShiftPlugin {
    fn proxy_and_adjust_timestamp<E, const N: usize>(&self, ph: PluginHandle<'_, Self>, event: E)
    where
        E: PrintEvent<N>,
    {
        ph.hook_print_attrs(event, Priority::Highest, |plugin, ph, attrs, args| {
            if plugin.inside_hook.get() {
                // Already inside hook, don't reprocess this event.
                return Eat::None;
            }

            let offset = plugin.offset_seconds.get();
            let new_time = if offset < 0 {
                attrs.time() - Duration::from_secs(offset.abs_diff(0))
            } else {
                attrs.time() + Duration::from_secs(offset.abs_diff(0))
            };
            let new_attrs = EventAttrs::new(new_time);

            plugin.inside_hook.set(true);
            if let Err(()) = ph.emit_print_attrs(E::default(), new_attrs, args) {
                ph.print(c"Failed to emit event.");
            }
            plugin.inside_hook.set(false);

            Eat::All
        });
    }
}

impl Plugin for TimeShiftPlugin {
    fn init(&self, ph: PluginHandle<'_, Self>) {
        ph.hook_command(
            c"timeshift",
            c"Usage: TIMESHIFT <seconds>, adjust timestamps of future messages",
            Priority::Normal,
            |plugin, ph, words| {
                match words[1].parse() {
                    Ok(offset) => {
                        plugin.offset_seconds.set(offset);
                        ph.print(format!("Timestamps will be shifted by {} seconds.", offset));
                    }
                    Err(e) => {
                        ph.print(format!("Invalid number of seconds: {}", e));
                    }
                }
                Eat::All
            },
        );

        self.proxy_and_adjust_timestamp(ph, ChannelMessage);
        self.proxy_and_adjust_timestamp(ph, MessageSend);
        self.proxy_and_adjust_timestamp(ph, PrivateMessage);

        ph.print(c"Time shift plugin loaded successfully!");
    }

    fn deinit(&self, ph: PluginHandle<'_, Self>) {
        ph.print(c"Unloading time shift plugin...");
    }
}

export_plugin!(
    TimeShiftPlugin,
    "Time Shift Plugin",
    "Adjust the timestamp of all channel messages with /timeshift",
    "1.0.0"
);
