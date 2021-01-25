pub mod capabilities;

mod client;
mod command;
mod command_batch;
mod objects;
mod traits;
mod utils;

pub use crate::client::Client;
pub use crate::command::{Command, CommandResult};
pub use crate::command_batch::CommandBatch;
pub use crate::objects::{
    Device, Devices, Location, Locations, PagedLocation, Room, Rooms, ScopedDevices,
};
pub use crate::traits::Capability;

pub use anyhow::Result;
pub use uuid::Uuid;
