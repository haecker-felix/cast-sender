use crate::Payload;
use serde::{Deserialize, Serialize};

use super::NamespaceUrn;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Receiver {
    GetStatus,
    ReceiverStatus(ReceiverStatusResponse),
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

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct ReceiverStatusResponse {
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<String>,
    pub muted: bool,
    pub level: f64,
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
    pub namespaces: Vec<NamespaceUrn>,
    pub session_id: String,
    pub status_text: String,
    pub transport_id: String,
    pub universal_app_id: String,
}
