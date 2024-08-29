use crate::Payload;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Heartbeat {
    Ping,
    Pong,
}

impl Into<Payload> for Heartbeat {
    fn into(self) -> Payload {
        Payload::Heartbeat(self.clone())
    }
}
