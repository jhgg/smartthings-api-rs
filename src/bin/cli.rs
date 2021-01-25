use smartthings_api::{
    capabilities::{switch::SwitchValue, ColorControl, Switch},
    Client, CommandBatch, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::with_personal_access_token(include_str!("../../.token"));
    // let device = client
    //     .devices()
    //     .get("9be16c40-f089-4c89-8ef4-42dff120840a")
    //     .await?;

    // dbg!(&device);

    // let switch: Switch = device.get_capability()?;
    // switch.toggle().await?;
    // switch.set(switch.get().await?.toggle()).execute().await?;

    // let cc: ColorControl = device.get_capability()?;
    // dbg!(cc.set_color(0, 100).await?);
    // // dbg!(cc.get_color().await?);
    // let level: SwitchLevel = device.get_capability()?;
    // level.set_level(50).await?;

    // let sc: RgbColorControl = device.get_synthesized_capability()?;
    // sc.set_rgb(100, 100, 200).await?;

    // let switch: Switch = device.get_capability()?;
    // let value = dbg!(switch.get().await?);
    // dbg!(switch.set(value.toggle()).await?);

    let mut batch = CommandBatch::new();

    for device in dbg!(client.devices().list().await?) {
        if device.label().to_lowercase().find("fan").is_some() {
            continue;
        }
        // println!("found fan: {:?}", device.label());
        // if let Ok(cap) = device.get_capability::<Switch>() {
        //     println!("Setting switch to off for device: {:?}", device.label());
        //     batch.add(cap.set(SwitchValue::On));
        // }
        // if let Ok(cap) = device.get_capability::<SwitchLevel>() {
        //     batch.add(cap.set_level(20));
        // }

        if let Ok(cap) = device.get_capability::<ColorControl>() {
            batch.add(cap.set_color(50, 100));
        } else {
            if let Ok(cap) = device.get_capability::<Switch>() {
                batch.add(cap.set(SwitchValue::Off));
            }
        }
    }

    batch.execute().await?;

    // let level: SwitchLevel = device.get_capability()?;
    // level.set_level(15).execute().await?;

    // let locations = client.locations().list().await?;
    // let paged_location = dbg!(locations.first().unwrap());
    // let location = dbg!(paged_location.full().await?);
    // let rooms = dbg!(location.rooms().list().await?);

    Ok(())
}
