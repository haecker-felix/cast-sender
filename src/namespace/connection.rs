use crate::Payload;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Connection {
    Connect,
    Close,
}

impl Into<Payload> for Connection {
    fn into(self) -> Payload {
        Payload::Connection(self.clone())
    }
}
