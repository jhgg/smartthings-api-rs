use smartthings_api::{
    capabilities::{switch::SwitchValue, Switch},
    Client, CommandBatch, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let token = std::fs::read_to_string("token.txt")?;
    let client = Client::with_personal_access_token(token);

    let mut batch = CommandBatch::new();

    for device in client.devices().list().await? {
        if let Ok(cap) = device.get_capability::<Switch>() {
            batch.add(cap.set(SwitchValue::Off));
        }
    }

    batch.execute().await?;
    Ok(())
}
