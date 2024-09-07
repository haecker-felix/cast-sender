#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_with;
#[macro_use]
extern crate derive_builder;

pub mod namespace;
mod proto;

mod app;
mod client;
mod error;
mod media_controller;
mod payload;
mod receiver;
mod shared;

pub use app::{App, AppId};
pub use client::{Client, Response};
pub use error::Error;
pub use media_controller::MediaController;
pub use payload::Payload;
pub use receiver::Receiver;
pub use shared::*;
