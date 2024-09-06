#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct Volume {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub control_type: Option<String>,
    pub muted: Option<bool>,
    pub level: Option<f64>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(setter(strip_option, into), default)]
pub struct Image {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
}
