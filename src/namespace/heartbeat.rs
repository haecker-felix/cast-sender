use crate::Payload;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Heartbeat {
    Ping,
    Pong,
}

impl From<Heartbeat> for Payload {
    fn from(val: Heartbeat) -> Self {
        Payload::Heartbeat(val.clone())
    }
}
