use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum_macros::{Display, EnumString};

use super::Volume;
use crate::Payload;

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
    #[strum(serialize = "urn:x-cast:com.google.cast.mudltizone")]
    Multizone,
    #[strum(default)]
    Unknown(String),
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Heartbeat {
    Ping,
    Pong,
}

impl Into<Payload> for Heartbeat {
    fn into(self) -> Payload {
        Payload::Heartbeat(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Connection {
    Connect,
    Close,
}

impl Into<Payload> for Connection {
    fn into(self) -> Payload {
        Payload::Connection(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Receiver {
    GetStatus,
    Launch(LaunchRequest),
    SetVolume(SetVolumeRequest),
    Stop(StopRequest),
}

impl Into<Payload> for Receiver {
    fn into(self) -> Payload {
        Payload::Receiver(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub struct Unknown {
    #[serde(skip)]
    pub namespace: NamespaceUrn,
    #[serde(flatten)]
    fields: HashMap<String, Value>,
}

impl Into<Payload> for Unknown {
    fn into(self) -> Payload {
        Payload::Unknown(self.clone())
    }
}

// ---------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LaunchRequest {
    pub app_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopRequest {
    pub session_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SetVolumeRequest {
    pub volume: Volume,
}
