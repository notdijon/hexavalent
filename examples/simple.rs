#![crate_type = "cdylib"]

use hexavalent::{export_plugin, HexchatPlugin, PluginHandle};

#[derive(Default)]
struct NoopPlugin;

impl HexchatPlugin for NoopPlugin {
    fn init(&self, ph: PluginHandle<'_>) {}
}

export_plugin!(NoopPlugin, name: "No-op Plugin", desc: "Does nothing.", version: "1.0.0");
