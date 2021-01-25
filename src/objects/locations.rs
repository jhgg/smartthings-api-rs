use uuid::Uuid;

use crate::{Client, Location, PagedLocation, Result, Rooms};

pub struct Locations {
    pub(crate) client: Client,
}

impl Locations {
    pub async fn list(&self) -> Result<Vec<PagedLocation>> {
        let response = self
            .client
            .http()
            .get("https://api.smartthings.com/v1/locations")
            .send()
            .await?;

        let locations: self::raw::ListResult = response.json().await?;

        let locations = locations
            .items
            .into_iter()
            .map(|data| PagedLocation {
                client: self.client.clone().into(),
                data,
            })
            .collect();

        Ok(locations)
    }

    pub async fn get(&self, location_id: Uuid) -> Result<Location> {
        let response = self
            .client
            .http()
            .get(&format!(
                "https://api.smartthings.com/v1/locations/{}",
                location_id
            ))
            .send()
            .await?;

        let data: crate::objects::location::raw::LocationData = response.json().await?;

        Ok(Location {
            client: self.client.clone().into(),
            data,
        })
    }

    pub fn rooms(&self, location_id: Uuid) -> Rooms {
        Rooms {
            client: self.client.clone(),
            location_id,
        }
    }
}

pub(crate) mod raw {
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub(crate) struct ListResult {
        pub(crate) items: Vec<crate::objects::location::raw::PagedLocationData>,
    }
}
