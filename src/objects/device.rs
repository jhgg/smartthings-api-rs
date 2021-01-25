use uuid::Uuid;

use crate::{utils::NoDebug, Capability, Client, Result};

#[derive(Debug)]
pub struct Device {
    pub(crate) client: NoDebug<Client>,
    pub(crate) data: self::raw::DeviceData,
}

impl Device {
    pub fn id(&self) -> Uuid {
        self.data.device_id
    }

    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn label(&self) -> &str {
        &self.data.label
    }

    pub fn get_capability<T: Capability>(&self) -> Result<T> {
        self.get_capability_with_component("main")
    }

    pub fn get_capability_with_component<T: Capability>(
        &self,
        component: impl AsRef<str>,
    ) -> Result<T> {
        let component = component.as_ref();
        let device_component_data = match self.data.components.iter().find(|x| x.id == component) {
            Some(component) => component,
            None => return Err(anyhow::format_err!("todo: component not found")),
        };

        match device_component_data
            .capabilities
            .iter()
            .find(|c| c.id == T::ID && c.version == T::VERSION)
        {
            Some(_) => Ok(T::with_device(&self, component)),
            None => Err(anyhow::format_err!("Capability not found")),
        }
    }

    // pub fn get_synthesized_capability<T: SynthesizedCapability>(&self) -> Result<T> {
    //     self.get_synthesized_capability_with_component("main")
    // }

    // pub fn get_synthesized_capability_with_component<T: SynthesizedCapability>(
    //     &self,
    //     component: impl AsRef<str>,
    // ) -> Result<T> {
    //     T::synthesize(self, component.as_ref())
    // }
}

pub(crate) mod raw {
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct DeviceData {
        pub(crate) device_id: Uuid,
        pub(crate) name: String,
        pub(crate) label: String,
        #[serde(default)]
        pub(crate) manufacturer_name: Option<String>,
        #[serde(default)]
        pub(crate) presentation_id: Option<String>,
        #[serde(default)]
        pub(crate) device_manufacturer_code: Option<String>,
        pub(crate) location_id: Uuid,
        #[serde(default)]
        pub(crate) owner_id: Option<Uuid>,
        #[serde(default)]
        pub(crate) room_id: Option<Uuid>,
        pub(crate) components: Vec<DeviceComponentData>,
        // pub(crate) child_devices: Vec<DeviceData>,
        #[serde(default)]
        pub(crate) profile: Option<DeviceProfileReference>,
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct DeviceProfileReference {
        pub(crate) id: Uuid,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct DeviceComponentData {
        pub(crate) id: String,
        #[serde(default)]
        pub(crate) label: Option<String>,
        pub(crate) capabilities: Vec<DeviceCapabilitiesData>,
        pub(crate) categories: Vec<DeviceCategory>,
        #[serde(default)]
        pub(crate) icon: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub(crate) struct DeviceCapabilitiesData {
        pub(crate) id: String,
        pub(crate) version: u32,
    }

    #[derive(Deserialize, Debug)]
    #[serde(tag = "categoryType", content = "name")]
    #[serde(rename_all = "lowercase")]
    pub(crate) enum DeviceCategory {
        Manufacturer(String),
        User(String),
    }
}
