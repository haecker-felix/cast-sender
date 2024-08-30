use super::Payload;
use crate::Application;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Receiver {
    // Request
    GetStatus,
    Launch(LaunchRequest),
    SetVolume(SetVolumeRequest),
    Stop(StopRequest),
    LaunchError(LaunchErrorResponse),

    // Response
    ReceiverStatus(ReceiverStatusResponse),
}

impl Receiver {
    pub fn set_volume_request(level: f64, muted: bool) -> Self {
        Self::SetVolume(SetVolumeRequest {
            volume: Volume {
                control_type: None,
                muted: Some(muted),
                level: Some(level),
            },
        })
    }

    pub fn launch_request(app_id: String) -> Self {
        Self::Launch(LaunchRequest { app_id })
    }

    pub fn stop_request(session_id: String) -> Self {
        Self::Stop(StopRequest { session_id })
    }
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
pub struct LaunchErrorResponse {
    pub reason: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub muted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Status {
    pub applications: Option<Vec<Application>>,
    pub is_active_input: Option<bool>,
    pub is_standby: Option<bool>,
    pub volume: Volume,
}
