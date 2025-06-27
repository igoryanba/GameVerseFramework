use gameverse_core::{plugins::GameVersePlugin, fcl::FiveMCompat};
use async_trait::async_trait;
use anyhow::Result;
use tracing::info;

#[derive(Default)]
pub struct {{plugin_name_camel}} {
    compat: Option<FiveMCompat>,
}

#[async_trait]
impl GameVersePlugin for {{plugin_name_camel}} {
    async fn initialize(&mut self, ctx: &gameverse_core::plugins::PluginContext) -> Result<()> {
        info!(target = "plugin", "{{plugin_name}} initialized");
        let compat = ctx.fivem_compat();
        compat.register_net_event("chatMessage", |args| {
            info!(target="plugin", "chatMessage: {:?}", args);
        }).await?;
        self.compat = Some(compat);
        Ok(())
    }

    async fn finalize(&mut self) -> Result<()> {
        info!(target="plugin", "{{plugin_name}} shutting down");
        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn GameVersePlugin {
    Box::into_raw(Box::new({{plugin_name_camel}}::default()))
}

#[no_mangle]
pub extern "C" fn destroy_plugin(plugin: *mut dyn GameVersePlugin) {
    if !plugin.is_null() {
        unsafe { let _ = Box::from_raw(plugin); }
    }
}
