use serde::{Deserialize, Serialize};

use super::namespace::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum Payload {
    Connection(Connection),
    Heartbeat(Heartbeat),
    Receiver(Receiver),
    Other(Other),
}

impl Payload {
    pub fn namespace(&self) -> NamespaceUrn {
        match self {
            Payload::Heartbeat(_) => NamespaceUrn::Heartbeat,
            Payload::Connection(_) => NamespaceUrn::Connection,
            Payload::Receiver(_) => NamespaceUrn::Receiver,
            Payload::Other(pl) => pl.namespace.clone(),
        }
    }
}
