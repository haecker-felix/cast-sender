use thiserror::Error;

use crate::namespace::media::ErrorReason;

use super::Response;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Input/Output error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Input/Output error: {0}")]
    AddrParseError(#[from] std::net::AddrParseError),

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

    #[error("Unable to launch app: {0}")]
    LaunchError(String),

    #[error("Unsupported Namespace")]
    UnsupportedNamespace,

    #[error("Media Channel Error: {0}")]
    MediaError(MediaError),

    #[error("Did not receive a matching response")]
    NoResponse,
}

#[derive(Error, Debug)]
pub enum MediaError {
    #[error("Invalid Request")]
    InvalidRequest(ErrorReason),

    #[error("Invalid Player State")]
    InvalidPlayerState,

    #[error("Load Failed")]
    LoadFailed,

    #[error("Load Cancelled")]
    LoadCancelled,
}
