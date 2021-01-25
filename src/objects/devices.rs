use reqwest::Url;
use uuid::Uuid;

use crate::{traits::IntoUuid, Client, Device, Result};

pub struct Devices {
    pub(crate) client: Client,
}

impl Devices {
    pub(crate) fn scoped(&self, location_id: Uuid) -> ScopedDevices {
        ScopedDevices {
            client: self.client.clone(),
            location_id,
            room_id: None,
        }
    }

    pub async fn list(&self) -> Result<Vec<Device>> {
        let response = self
            .client
            .http()
            .get("https://api.smartthings.com/v1/devices")
            .send()
            .await?;
        let devices: self::raw::ListResult = response.json().await?;

        let devices = devices
            .items
            .into_iter()
            .map(|data| Device {
                client: self.client.clone().into(),
                data,
            })
            .collect();

        Ok(devices)
    }

    pub async fn get(&self, device_id: impl IntoUuid) -> Result<Device> {
        let url = format!(
            "https://api.smartthings.com/v1/devices/{}",
            device_id.into_uuid()?
        );
        let response = self.client.http().get(&url).send().await?;
        let data: crate::objects::device::raw::DeviceData = response.json().await?;

        Ok(Device {
            client: self.client.clone().into(),
            data,
        })
    }
}

pub struct ScopedDevices {
    client: Client,
    location_id: Uuid,
    room_id: Option<Uuid>,
}

impl ScopedDevices {
    pub(crate) fn in_room(mut self, room_id: Uuid) -> Self {
        self.room_id = Some(room_id);
        self
    }

    pub async fn list(&self) -> Result<Vec<Device>> {
        let url = Url::parse_with_params(
            "https://api.smartthings.com/v1/devices",
            &[("locationId", self.location_id.to_string())],
        )?;

        let response = self.client.http().get(url).send().await?;
        let devices: self::raw::ListResult = response.json().await?;

        let devices = devices
            .items
            .into_iter()
            .filter(|data| match &self.room_id {
                room_id @ Some(_) => &data.room_id == room_id,
                None => true,
            })
            .map(|data| Device {
                client: self.client.clone().into(),
                data,
            })
            .collect();

        Ok(devices)
    }
}

mod raw {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub(crate) struct ListResult {
        pub(crate) items: Vec<crate::objects::device::raw::DeviceData>,
    }
}
