use serde::{Deserialize, Serialize};

use super::namespace::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Payload {
    Connection(Connection),
    Heartbeat(Heartbeat),
    Receiver(Receiver),
    Unknown(Unknown),
}

impl Payload {
    pub fn namespace(&self) -> NamespaceUrn {
        match self {
            Payload::Heartbeat(_) => NamespaceUrn::Heartbeat,
            Payload::Connection(_) => NamespaceUrn::Connection,
            Payload::Receiver(_) => NamespaceUrn::Receiver,
            Payload::Unknown(pl) => pl.namespace.clone(),
        }
    }
}

// --------------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReceiverStatusResponse {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub applications: Option<Vec<Application>>,
    pub is_active_input: Option<bool>,
    pub is_standby: Option<bool>,
    pub volume: Volume,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub app_id: String,
    pub app_type: String,
    pub display_name: String,
    pub icon_url: String,
    pub is_idle_screen: bool,
    pub launched_from_cloud: bool,
    // namespaces
    pub session_id: String,
    pub status_text: String,
    pub transport_id: String,
    pub universal_app_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<String>,
    pub muted: bool,
    pub level: f64,
}
