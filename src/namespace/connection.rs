use crate::Payload;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum Connection {
    Connect,
    Close,
}

impl From<Connection> for Payload {
    fn from(val: Connection) -> Self {
        Payload::Connection(val.clone())
    }
}
