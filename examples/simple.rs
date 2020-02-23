use hexavalent::{export_plugin, Plugin, PluginHandle};

#[derive(Default)]
struct SimplePlugin;

impl Plugin for SimplePlugin {
    fn init(&self, ph: PluginHandle<'_, Self>) {
        ph.print("Plugin loaded successfully!\0");

        // todo make this a simple message counter
    }

    fn deinit(&self, ph: PluginHandle<'_, Self>) {
        ph.print("Unloading plugin...\0");
    }
}

export_plugin!(SimplePlugin, "Simple Example", "Doesn't do much.", "1.0.0");
