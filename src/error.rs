use thiserror::Error;

use super::Response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Input/Output error: {0}")]
    Io(#[from] std::io::Error),

    #[error("TLS error: {0}")]
    Tls(#[from] async_native_tls::Error),

    #[error("Decode error: {0}")]
    Decode(#[from] prost::DecodeError),

    #[error("Deserialize error: {0}")]
    Deserialize(#[from] serde_json::Error),

    #[error("Receive error: {0}")]
    Receive(#[from] async_channel::RecvError),

    #[error("Send error: {0}")]
    Send(#[from] async_channel::SendError<Response>),

    #[error("Did not receive request response")]
    ResponseTimeout,

    #[error("Not connected with receiver")]
    NoConnection,
}
