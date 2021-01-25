use uuid::Uuid;

use crate::{utils::NoDebug, Client, Result, Rooms, ScopedDevices};

#[derive(Debug)]
pub struct PagedLocation {
    pub(crate) client: NoDebug<Client>,
    pub(crate) data: self::raw::PagedLocationData,
}

impl PagedLocation {
    pub fn location_id(&self) -> Uuid {
        self.data.location_id
    }

    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn parent(&self) -> Option<LocationParent> {
        self.data.parent.clone()
    }

    pub async fn full(&self) -> Result<Location> {
        self.client.locations().get(self.data.location_id).await
    }

    pub fn rooms(&self) -> Rooms {
        self.client.locations().rooms(self.location_id())
    }

    pub fn devices(&self) -> ScopedDevices {
        self.client.devices().scoped(self.location_id())
    }
}

#[derive(Debug)]
pub struct Location {
    pub(crate) client: NoDebug<Client>,
    pub(crate) data: self::raw::LocationData,
}

impl Location {
    pub fn location_id(&self) -> Uuid {
        self.data.location_id
    }

    pub fn name(&self) -> &str {
        &self.data.name
    }

    pub fn parent(&self) -> Option<self::raw::LocationParent> {
        self.data.parent.clone()
    }

    pub fn rooms(&self) -> Rooms {
        self.client.locations().rooms(self.location_id())
    }

    pub fn devices(&self) -> ScopedDevices {
        self.client.devices().scoped(self.location_id())
    }
}

pub use self::raw::LocationParent;

pub(crate) mod raw {
    use chrono::serde::ts_milliseconds;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct PagedLocationData {
        pub(crate) location_id: Uuid,
        pub(crate) name: String,
        pub(crate) parent: Option<LocationParent>,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct LocationData {
        pub(crate) location_id: Uuid,
        pub(crate) name: String,
        pub(crate) country_code: String,
        pub(crate) latitude: f64,
        pub(crate) longitude: f64,
        pub(crate) region_radius: i32,
        pub(crate) temperature_scale: TemperatureScale,
        pub(crate) time_zone_id: String,
        pub(crate) locale: String,
        // pub(crate) background_image: String,
        #[serde(with = "ts_milliseconds")]
        pub(crate) created: DateTime<Utc>,
        #[serde(with = "ts_milliseconds")]
        pub(crate) last_modified: DateTime<Utc>,
        pub(crate) parent: Option<LocationParent>,
    }

    #[derive(Deserialize, Debug)]
    pub enum TemperatureScale {
        #[serde(rename = "C")]
        Celsius,
        #[serde(rename = "F")]
        Fahrenheit,
    }

    #[derive(Deserialize, Debug, Clone, Copy)]
    #[serde(tag = "type", content = "id")]
    #[serde(rename_all = "UPPERCASE")]
    pub enum LocationParent {
        LocationGroup(Uuid),
        Account(Uuid),
    }
}
