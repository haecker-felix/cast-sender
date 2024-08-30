use crate::{Error, Payload, Receiver, Response};

use super::NamespaceUrn;

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

impl Application {
    pub fn receiver() -> Self {
        Self {
            transport_id: "receiver-0".into(),
            namespaces: vec![
                NamespaceUrn::Connection,
                NamespaceUrn::Heartbeat,
                NamespaceUrn::Receiver,
                NamespaceUrn::DeviceAuth,
            ],
            ..Default::default()
        }
    }
}
