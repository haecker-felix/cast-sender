use crate::{app::AppId, App, Payload, Volume};

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

    pub fn launch_request(app_id: AppId) -> Self {
        Self::Launch(LaunchRequest {
            app_id: app_id.to_string(),
        })
    }

    pub fn stop_request(session_id: String) -> Self {
        Self::Stop(StopRequest { session_id })
    }
}

impl From<Receiver> for Payload {
    fn from(val: Receiver) -> Self {
        Payload::Receiver(val.clone())
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
pub struct Status {
    pub applications: Option<Vec<App>>,
    pub is_active_input: Option<bool>,
    pub is_standby: Option<bool>,
    pub volume: Volume,
}
