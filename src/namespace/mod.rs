use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};

use crate::Payload;

mod connection;
mod heartbeat;
mod receiver;

pub use connection::*;
pub use heartbeat::*;
pub use receiver::*;

#[derive(EnumString, Display, Debug, Clone, Default)]
pub enum NamespaceUrn {
    #[default]
    #[strum(serialize = "urn:x-cast:com.google.cast.tp.connection")]
    Connection,
    #[strum(serialize = "urn:x-cast:com.google.cast.tp.heartbeat")]
    Heartbeat,
    #[strum(serialize = "urn:x-cast:com.google.cast.receiver")]
    Receiver,
    #[strum(serialize = "urn:x-cast:com.google.cast.tp.deviceauth")]
    DeviceAuth,
    #[strum(serialize = "urn:x-cast:com.google.cast.multizone")]
    Multizone,
    #[strum(default)]
    Other(String),
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub struct Other {
    #[serde(skip)]
    pub namespace: NamespaceUrn,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

impl Into<Payload> for Other {
    fn into(self) -> Payload {
        Payload::Other(self.clone())
    }
}
