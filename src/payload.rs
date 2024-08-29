use serde::{Deserialize, Serialize};

use super::namespace::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Payload {
    Connection(Connection),
    Heartbeat(Heartbeat),
    Multizone(Multizone),
    Receiver(Receiver),
    Custom(Custom),
}

impl Payload {
    pub fn namespace(&self) -> NamespaceUrn {
        match self {
            Payload::Connection(_) => NamespaceUrn::Connection,
            Payload::Heartbeat(_) => NamespaceUrn::Heartbeat,
            Payload::Multizone(_) => NamespaceUrn::Multizone,
            Payload::Receiver(_) => NamespaceUrn::Receiver,
            Payload::Custom(pl) => pl.namespace.clone(),
        }
    }
}
