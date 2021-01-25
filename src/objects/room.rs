use crate::{utils::NoDebug, Client, ScopedDevices};

#[derive(Debug)]
pub struct Room {
    pub(crate) client: NoDebug<Client>,
    pub(crate) data: self::raw::RoomData,
}

impl Room {
    pub fn devices(&self) -> ScopedDevices {
        self.client
            .devices()
            .scoped(self.data.location_id)
            .in_room(self.data.room_id)
    }
}

pub(crate) mod raw {
    use chrono::serde::ts_milliseconds;
    use chrono::{DateTime, Utc};
    use serde::Deserialize;
    use uuid::Uuid;

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "camelCase")]
    pub(crate) struct RoomData {
        pub(crate) room_id: Uuid,
        pub(crate) location_id: Uuid,
        pub(crate) name: String,
        // pub(crate) background_image: String,
        #[serde(with = "ts_milliseconds")]
        pub(crate) created: DateTime<Utc>,
        #[serde(with = "ts_milliseconds")]
        pub(crate) last_modified: DateTime<Utc>,
    }
}
