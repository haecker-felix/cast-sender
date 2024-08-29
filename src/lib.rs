#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/openscreen.cast.proto.rs"));
}

mod client;
mod error;
mod namespace;
mod payload;
mod receiver;

pub use client::{Client, Response};
pub use error::Error;
pub use namespace::*;
pub use payload::Payload;
pub use receiver::Receiver;
