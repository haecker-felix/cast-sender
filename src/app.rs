use std::str::FromStr;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use strum_macros::{Display, EnumString};

use crate::namespace::NamespaceUrn;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct App {
    pub app_id: AppId,
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

impl App {
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

#[derive(EnumString, Display, Debug, Clone, Default, PartialEq, Eq)]
pub enum AppId {
    #[default]
    #[strum(serialize = "CC1AD845")]
    DefaultMediaReceiver,
    #[strum(serialize = "E8C28D3C")]
    Backdrop,
    #[strum(serialize = "233637DE")]
    YouTube,
    #[strum(default)]
    Custom(String),
}

impl<'de> Deserialize<'de> for AppId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let id = String::deserialize(deserializer)?;
        AppId::from_str(&id).map_err(serde::de::Error::custom)
    }
}

impl Serialize for AppId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.to_string().serialize(serializer)
    }
}
