use uuid::Uuid;

use crate::{Client, Result, Room};

pub struct Rooms {
    pub(crate) client: Client,
    pub(crate) location_id: Uuid,
}

impl Rooms {
    pub async fn list(&self) -> Result<Vec<Room>> {
        let response = self
            .client
            .http()
            .get(&format!(
                "https://api.smartthings.com/v1/locations/{}/rooms",
                self.location_id
            ))
            .send()
            .await?;

        let rooms: self::raw::ListRoomsResult = response.json().await?;

        let locations = rooms
            .items
            .into_iter()
            .map(|data| Room {
                client: self.client.clone().into(),
                data,
            })
            .collect();

        Ok(locations)
    }

    pub async fn get(&self, room_id: Uuid) -> Result<Room> {
        let response = self
            .client
            .http()
            .get(&format!(
                "https://api.smartthings.com/v1/locations/{}/rooms/{}",
                self.location_id, room_id
            ))
            .send()
            .await?;

        let data: crate::objects::room::raw::RoomData = response.json().await?;

        Ok(Room {
            client: self.client.clone().into(),
            data,
        })
    }
}

pub(crate) mod raw {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub(crate) struct ListRoomsResult {
        pub(crate) items: Vec<crate::objects::room::raw::RoomData>,
    }
}
