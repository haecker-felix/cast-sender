use crate::namespace::{
    connection::*, heartbeat::*, media::*, multizone::*, receiver::*, Custom, NamespaceUrn,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Payload {
    Connection(Connection),
    Heartbeat(Heartbeat),
    Media(Media),
    Multizone(Multizone),
    Receiver(Receiver),
    // Fallback -> Needs to be last enum variant!
    Custom(Custom),
}

impl Payload {
    pub fn namespace(&self) -> NamespaceUrn {
        match self {
            Payload::Connection(_) => NamespaceUrn::Connection,
            Payload::Heartbeat(_) => NamespaceUrn::Heartbeat,
            Payload::Media(_) => NamespaceUrn::Media,
            Payload::Multizone(_) => NamespaceUrn::Multizone,
            Payload::Receiver(_) => NamespaceUrn::Receiver,
            Payload::Custom(pl) => pl.namespace.clone(),
        }
    }
}
