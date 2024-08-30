use crate::Payload;

use super::Volume;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Media {
    // Request
    Load(LoadRequest),
    Pause(PauseRequest),
    Seek, // TODO
    Stop(StopRequest),
    Play(PlayRequest),
    GetStatus(GetStatusRequest),
    Volume, // TODO

    // Response
    InvalidPlayerState,
    LoadFailed,
    LoadCancelled,
    InvalidRequest(InvalidRequestResponse),
}

impl Media {
    pub fn load(
        media: MediaInformation,
        autoplay: Option<bool>,
        current_time: Option<f64>,
    ) -> Self {
        Self::Load(LoadRequest {
            media,
            autoplay,
            current_time,
        })
    }
}

impl Into<Payload> for Media {
    fn into(self) -> Payload {
        Payload::Media(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct LoadRequest {
    pub media: MediaInformation,
    pub autoplay: Option<bool>,
    pub current_time: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PauseRequest {
    pub media_session_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct StopRequest {
    pub media_session_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayRequest {
    pub media_session_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct GetStatusRequest {
    pub media_session_id: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct VolumeRequest {
    pub volume: Volume,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct MediaInformation {
    pub content_id: String,
    pub stream_type: StreamType,
    pub content_type: String,
    // TODO pub metadata: Metadata,
    pub duration: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum StreamType {
    #[default]
    None,
    Buffered,
    Live,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct InvalidRequestResponse {
    pub reason: String,
}
