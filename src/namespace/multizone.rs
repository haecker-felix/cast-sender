use crate::Payload;

use crate::Volume;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Multizone {
    DeviceUpdated(DeviceResponse),
}

impl Into<Payload> for Multizone {
    fn into(self) -> Payload {
        Payload::Multizone(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct DeviceResponse {
    pub device: Device,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub capabilities: u32,
    pub device_id: String,
    pub name: String,
    pub volume: Volume,
}
