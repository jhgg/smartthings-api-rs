use std::collections::HashMap;

use uuid::Uuid;

use crate::{Command, CommandResult, Result};

#[derive(Default)]
pub struct CommandBatch {
    commands_by_device: HashMap<Uuid, Vec<Command>>,
}

impl CommandBatch {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, command: Command) {
        self.commands_by_device
            .entry(command.device_id)
            .or_default()
            .push(command);
    }

    pub async fn execute(self) -> Result<HashMap<Uuid, Vec<CommandResult>>> {
        let mut result = HashMap::with_capacity(self.commands_by_device.len());

        for (device_id, commands) in self.commands_by_device {
            let client = commands
                .first()
                .expect("invariant: must have at least one command")
                .client
                .clone();

            let url = format!(
                "https://api.smartthings.com/v1/devices/{}/commands",
                device_id
            );

            let commands = dbg!(serde_json::json!({ "commands": commands }));
            let response = dbg!(client.http().post(&url).json(&commands).send().await?);
            let results: crate::command::CommandResults = dbg!(response.json().await?);
            result.insert(device_id, results.results);
        }

        Ok(result)
    }
}
