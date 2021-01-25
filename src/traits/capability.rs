use std::marker::PhantomData;

use crate::{utils::NoDebug, Client, Device, Result};

use serde::de::DeserializeOwned;
use uuid::Uuid;

pub trait Capability: Sized {
    const ID: &'static str;
    const VERSION: u32;

    fn with_device(device: &Device, component: &str) -> Self;

    #[doc(hidden)]
    fn __meta(&self) -> &CapabilityMeta<Self>;
}

pub struct CapabilityMeta<C> {
    pub(crate) client: NoDebug<Client>,
    pub(crate) device_id: Uuid,
    pub(crate) component: String,
    pub(crate) _phantom: PhantomData<C>,
}

impl<C> CapabilityMeta<C>
where
    C: Capability,
{
    pub(crate) fn with_device(device: &Device, component: &str) -> Self {
        Self {
            client: device.client.clone(),
            device_id: device.id(),
            component: component.to_owned(),
            _phantom: PhantomData,
        }
    }

    pub(crate) async fn query_status<T: DeserializeOwned>(&self) -> Result<T> {
        let url = format!(
            "https://api.smartthings.com/v1/devices/{}/components/{}/capabilities/{}/status",
            self.device_id,
            self.component,
            C::ID
        );

        let response = self.client.http().get(&url).send().await?;
        Ok(response.json().await?)
    }
}
