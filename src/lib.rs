#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_with;
#[macro_use]
extern crate derive_builder;

mod proto {
    include!(concat!(env!("OUT_DIR"), "/openscreen.cast.proto.rs"));
}

mod application;
mod client;
mod error;
mod media_controller;
pub mod namespace;
mod payload;
mod receiver;
mod shared;

pub use application::Application;
pub use client::{Client, Response};
pub use error::Error;
pub use media_controller::MediaController;
pub use payload::Payload;
pub use receiver::Receiver;
pub use shared::*;
