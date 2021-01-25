use serde::Deserialize;

use crate::{traits::CapabilityMeta, Capability, Command, CommandResult, Device, Result};

pub struct Switch(CapabilityMeta<Self>);

impl Capability for Switch {
    const ID: &'static str = "switch";
    const VERSION: u32 = 1;

    fn with_device(device: &Device, component: &str) -> Self {
        Self(CapabilityMeta::with_device(device, component))
    }

    fn __meta(&self) -> &CapabilityMeta<Self> {
        &self.0
    }
}

#[derive(Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SwitchValue {
    On,
    Off,
}

impl SwitchValue {
    pub fn toggle(self) -> Self {
        match self {
            SwitchValue::On => SwitchValue::Off,
            SwitchValue::Off => SwitchValue::On,
        }
    }

    pub(crate) fn command(&self) -> &'static str {
        match self {
            SwitchValue::On => "on",
            SwitchValue::Off => "off",
        }
    }
}

impl Switch {
    pub fn set(&self, value: SwitchValue) -> Command {
        Command::for_capability(self, value.command(), vec![])
    }

    pub async fn get(&self) -> Result<SwitchValue> {
        #[derive(Deserialize)]
        struct Result {
            switch: ResultInner,
        }
        #[derive(Deserialize)]
        struct ResultInner {
            value: SwitchValue,
        }

        let result: Result = self.__meta().query_status().await?;
        Ok(result.switch.value)
    }

    pub async fn on(&self) -> Result<CommandResult> {
        self.set(SwitchValue::On).execute().await
    }

    pub async fn off(&self) -> Result<CommandResult> {
        self.set(SwitchValue::Off).execute().await
    }

    pub async fn toggle(&self) -> Result<CommandResult> {
        let current = self.get().await?;
        self.set(current.toggle()).execute().await
    }
}
