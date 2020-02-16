#![crate_type = "cdylib"]

use hexavalent::{export_plugin, HexchatPlugin, PluginHandle};

#[derive(Default)]
struct SimplePlugin;

impl HexchatPlugin for SimplePlugin {
    fn init(&self, ph: PluginHandle<'_>) {
        ph.print("Plugin loaded successfully!\0");
    }

    fn deinit(&self, ph: PluginHandle<'_>) {
        ph.print("Unloading plugin...\0");
    }
}

export_plugin!(SimplePlugin, name: "Simple Example Plugin", desc: "Doesn't do much.", version: "1.0.0");
