WIP: Rust API for smart things.


### Example

Turn on all the freaking lights...

```rust
use smartthings_api::{Client, Result, CommandBatch, capabilities::{Switch, switch::SwitchValue}};

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
```