use serde_json::json;

use crate::{traits::CapabilityMeta, Capability, Command, Device};

pub struct SwitchLevel(CapabilityMeta<Self>);

impl Capability for SwitchLevel {
    const ID: &'static str = "switchLevel";
    const VERSION: u32 = 1;

    fn with_device(device: &Device, component: &str) -> Self {
        Self(CapabilityMeta::with_device(device, component))
    }

    fn __meta(&self) -> &CapabilityMeta<Self> {
        &self.0
    }
}

impl SwitchLevel {
    pub fn set_level(&self, level: u8) -> Command {
        Command::for_capability(self, "setLevel", vec![json!(level)])
    }
}
