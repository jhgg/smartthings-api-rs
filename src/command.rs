use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{utils::NoDebug, Capability, Client, Result};

#[derive(Serialize)]
pub struct Command {
    #[serde(skip)]
    pub(crate) client: NoDebug<Client>,
    #[serde(skip)]
    pub(crate) device_id: Uuid,
    component: Cow<'static, str>,
    capability: &'static str,
    command: Cow<'static, str>,
    arguments: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum CommandStatus {
    Accepted,
    Completed,
    Failed,
}

#[derive(Deserialize, Debug)]
pub struct CommandResult {
    pub id: Uuid,
    pub status: CommandStatus,
}

#[derive(Deserialize, Debug)]
pub(crate) struct CommandResults {
    pub(crate) results: Vec<CommandResult>,
}

impl Command {
    pub(crate) fn for_capability<T: Capability>(
        cap: &T,
        command: &'static str,
        arguments: Vec<serde_json::Value>,
    ) -> Self {
        let meta = cap.__meta();
        Self {
            client: meta.client.clone().into(),
            device_id: meta.device_id,
            capability: T::ID,
            component: meta.component.to_owned().into(),
            command: command.into(),
            arguments,
        }
    }

    pub async fn execute(self) -> Result<CommandResult> {
        let url = format!(
            "https://api.smartthings.com/v1/devices/{}/commands",
            self.device_id
        );

        let commands = serde_json::json!({ "commands": [self] });
        let response = self.client.http().post(&url).json(&commands).send().await?;
        let results: CommandResults = response.json().await?;
        let result = results
            .results
            .into_iter()
            .next()
            .ok_or_else(|| anyhow::format_err!("todo: no result?!"))?;

        Ok(result)
    }
}
