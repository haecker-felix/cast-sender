use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use strum_macros::{Display, EnumString};

use crate::Payload;

mod connection;
mod heartbeat;
mod multizone;
mod receiver;

pub use connection::*;
pub use heartbeat::*;
pub use multizone::*;
pub use receiver::*;

#[derive(EnumString, Display, Debug, Clone, Default)]
pub enum NamespaceUrn {
    #[strum(serialize = "urn:x-cast:com.google.cast.cac")]
    Cac,
    #[default]
    #[strum(serialize = "urn:x-cast:com.google.cast.tp.connection")]
    Connection,
    #[strum(default)]
    Custom(String),
    #[strum(serialize = "urn:x-cast:com.google.cast.debugoverlay")]
    DebugOverlay,
    #[strum(serialize = "urn:x-cast:com.google.cast.tp.deviceauth")]
    DeviceAuth,
    #[strum(serialize = "urn:x-cast:com.google.cast.tp.heartbeat")]
    Heartbeat,
    #[strum(serialize = "urn:x-cast:com.google.cast.multizone")]
    Multizone,
    #[strum(serialize = "urn:x-cast:com.google.cast.receiver")]
    Receiver,
    #[strum(serialize = "urn:x-cast:com.google.cast.remotecontrol")]
    RemoteControl,
    #[strum(serialize = "urn:x-cast:com.google.cast.sse")]
    Sse,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub struct Custom {
    #[serde(skip)]
    pub namespace: NamespaceUrn,
    #[serde(flatten)]
    pub fields: HashMap<String, Value>,
}

impl Into<Payload> for Custom {
    fn into(self) -> Payload {
        Payload::Custom(self.clone())
    }
}

impl<'de> Deserialize<'de> for NamespaceUrn {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Namespace {
            name: String,
        }

        let ns = Namespace::deserialize(deserializer)?;
        NamespaceUrn::from_str(&ns.name).map_err(serde::de::Error::custom)
    }
}

impl Serialize for NamespaceUrn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        #[derive(Serialize)]
        pub struct Namespace {
            name: String,
        }

        Namespace {
            name: self.to_string(),
        }
        .serialize(serializer)
    }
}
