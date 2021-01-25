use serde_json::json;

use crate::{traits::CapabilityMeta, Capability, Command, Device, Result};

pub struct ColorControl(CapabilityMeta<Self>);

impl Capability for ColorControl {
    const ID: &'static str = "colorControl";
    const VERSION: u32 = 1;

    fn with_device(device: &Device, component: &str) -> Self {
        Self(CapabilityMeta::with_device(device, component))
    }

    fn __meta(&self) -> &CapabilityMeta<Self> {
        &self.0
    }
}

impl ColorControl {
    pub fn set_hue(&self, hue: u32) -> Command {
        Command::for_capability(self, "setHue", vec![json!(hue)])
    }

    pub fn set_saturation(&self, saturation: u32) -> Command {
        Command::for_capability(self, "setSaturation", vec![json!(saturation)])
    }

    pub fn set_color(&self, hue: u32, saturation: u32) -> Command {
        Command::for_capability(
            self,
            "setColor",
            vec![json!({
                "hue": hue,
                "saturation": saturation
            })],
        )
    }

    // todo: do correctly
    pub async fn get_color(&self) -> Result<serde_json::Value> {
        self.__meta().query_status().await
    }
}
