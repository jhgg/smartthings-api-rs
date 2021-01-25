use uuid::Uuid;

pub trait IntoUuid {
    fn into_uuid(self) -> Result<Uuid, uuid::Error>;
}

impl IntoUuid for Uuid {
    fn into_uuid(self) -> Result<Uuid, uuid::Error> {
        Ok(self)
    }
}

impl IntoUuid for &str {
    fn into_uuid(self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(self)
    }
}
