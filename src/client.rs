use std::sync::Arc;

use async_native_tls::{Host, TlsConnector, TlsStream};
use async_net::{AsyncToSocketAddrs, TcpStream};
use futures_util::io::{ReadHalf, WriteHalf};
use futures_util::{AsyncReadExt, AsyncWriteExt};
use prost::Message;
use serde::{Deserialize, Serialize};
use smol::lock::Mutex;

use super::proto;
use super::{Error, NamespaceUrn, Payload};

#[derive(Debug, Clone)]
pub struct Client {
    read_stream: Arc<Mutex<ReadHalf<TlsStream<TcpStream>>>>,
    write_stream: Arc<Mutex<WriteHalf<TlsStream<TcpStream>>>>,
}

impl Client {
    pub async fn connect<A: AsyncToSocketAddrs + Into<Host> + Clone>(
        addr: A,
    ) -> Result<Self, Error> {
        // Casts devices are using self signed certs
        let tls_connector = TlsConnector::new().danger_accept_invalid_certs(true);
        let tcp_stream = TcpStream::connect(&addr).await?;

        let tls_stream = tls_connector
            .connect(addr.clone(), tcp_stream.clone())
            .await?;

        let (read_stream, write_stream) = tls_stream.split();

        Ok(Self {
            read_stream: Arc::new(Mutex::new(read_stream)),
            write_stream: Arc::new(Mutex::new(write_stream)),
        })
    }

    pub async fn receive(&self) -> Result<Response, Error> {
        let mut read_stream = self.read_stream.lock().await;

        // The first package is a u32 specifying the packet length....
        let mut buf: [u8; 4] = [0; 4];
        read_stream.read_exact(&mut buf).await?;
        let len = u32::from_be_bytes(buf);

        // ... then get the actual package with the specified length
        let mut buf: Vec<u8> = vec![0; len as usize];
        read_stream.read_exact(&mut buf).await?;

        let msg: proto::CastMessage = proto::CastMessage::decode(&buf[..])?;
        let ns: NamespaceUrn = msg.namespace.parse().unwrap();
        let pl: PayloadData = serde_json::from_str(msg.payload_utf8())?;

        debug!("Received [{:?}]: {:#?}", ns, pl);
        Ok(Response {
            request_id: pl.request_id,
            namespace: ns,
            payload: pl.data,
        })
    }

    pub async fn send<P: Into<Payload>>(&self, payload: P) -> Result<(), Error> {
        self.send_full(payload.into(), None).await
    }

    pub async fn send_full(&self, payload: Payload, request_id: Option<u32>) -> Result<(), Error> {
        debug!(
            "Send [{:?}]: {}",
            payload.namespace(),
            serde_json::to_string(&payload).unwrap()
        );

        let namespace = payload.namespace();
        let payload_data = PayloadData {
            request_id,
            data: payload,
        };

        let msg = proto::CastMessage {
            protocol_version: proto::cast_message::ProtocolVersion::Castv210.into(),
            source_id: "sender-0".into(),
            destination_id: "receiver-0".into(),
            namespace: namespace.to_string(),
            payload_type: proto::cast_message::PayloadType::String.into(),
            payload_utf8: Some(serde_json::to_string(&payload_data).unwrap()),
            payload_binary: None,
            continued: None,
            remaining_length: None,
        };

        let mut write_stream = self.write_stream.lock().await;
        let len: u32 = msg.encoded_len().try_into().unwrap();

        // First send package length
        write_stream.write_all(&len.to_be_bytes()).await?;

        // Then the actual package
        write_stream.write_all(&msg.encode_to_vec()).await?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PayloadData {
    #[serde(skip_serializing_if = "Option::is_none")]
    request_id: Option<u32>,
    #[serde(flatten)]
    data: Payload,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub request_id: Option<u32>,
    // Probably not strictly necessary, since the namespace can be derived
    // using the payload, but this may not have any guarantee of correctness,
    // since the namespace may differ from the deserialized enum variant.
    pub namespace: NamespaceUrn,
    pub payload: Payload,
}
