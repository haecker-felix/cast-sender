use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use async_channel::Sender;
use async_native_tls::Host;
use async_net::AsyncToSocketAddrs;
use smol::lock::Mutex;
use smol_timeout::TimeoutExt;

use crate::namespace::{connection::*, heartbeat::*, receiver, receiver::*};
use crate::{Application, Response, Volume};

use super::{Client, Error, Payload};

#[derive(Debug, Clone)]
pub struct Receiver {
    client: Arc<Mutex<Option<Client>>>,
    platform: Application,

    // Ids for request messages which get incremented
    request_id: Arc<Mutex<u32>>,
    requests: Arc<Mutex<HashMap<u32, Sender<Response>>>>,
}

impl Receiver {
    pub fn new() -> Self {
        Self {
            client: Arc::default(),
            platform: Application::receiver(),
            request_id: Arc::default(),
            requests: Arc::default(),
        }
    }

    pub async fn connect<A: AsyncToSocketAddrs + Into<Host> + Clone>(
        &self,
        addr: A,
    ) -> Result<(), Error> {
        let client = Client::connect(addr).await?;
        self.client.lock().await.replace(client.clone());

        // Establish virtual connection with cast receiver
        self.send(&self.platform, Connection::Connect).await?;

        // Ensure we're successfully connected by doing a ping <-> pong sequence
        self.send(&self.platform, Heartbeat::Ping).await?;
        client.receive().await?;

        // Spawn own task to receive messages from the receiver
        let d = self.clone();
        smol::spawn(async move {
            loop {
                if let Some(client) = d.client().await {
                    match client.receive().await {
                        Ok(response) => {
                            if let Err(err) = d.process_response(response).await {
                                warn!("Unable to process received message: {}", err.to_string())
                            }
                        }
                        Err(err) => {
                            error!("Unable to receive message: {}", err.to_string());
                            d.disconnect().await;
                            break;
                        }
                    }
                } else {
                    debug!("No client available, stop receiving.");
                    break;
                }
            }
        })
        .detach();

        Ok(())
    }

    /// Only closes the underlying connection, does not stop any running applications.
    pub async fn disconnect(&self) {
        // Try to close the virtual connection, but don't care about the result
        let _ = self.send(&self.platform, Connection::Close).await;

        let mut client = self.client.lock().await;
        *client = None;

        // Reset requestId counter
        *self.request_id.lock().await = 0;
    }

    pub async fn is_connected(&self) -> bool {
        self.client().await.is_some()
    }

    /// Currently running applications
    pub async fn applications(&self) -> Result<Vec<Application>, Error> {
        Ok(self.status().await?.applications.unwrap_or_default())
    }

    pub async fn launch_app(&self, app_id: String) -> Result<Application, Error> {
        let response = self
            .send_request(
                &self.platform,
                receiver::Receiver::launch_request(app_id.clone()),
            )
            .await?;

        if let Payload::Receiver(payload) = response.payload {
            if let receiver::Receiver::LaunchError(LaunchErrorResponse { reason }) = payload {
                return Err(Error::LaunchError(reason));
            }

            if let receiver::Receiver::ReceiverStatus(ReceiverStatusResponse { status }) = payload {
                if let Some(apps) = status.applications {
                    for app in apps {
                        if app.app_id == app_id {
                            // Establish new virtual connection to be able to send/receive app specific payloads
                            self.send(&app, Connection::Connect).await?;
                            return Ok(app);
                        }
                    }
                }
            }
        }

        Err(Error::NoResponse)
    }

    pub async fn stop_app(&self, app: &Application) -> Result<(), Error> {
        self.send_request(
            &self.platform,
            receiver::Receiver::stop_request(app.session_id.clone()),
        )
        .await?;
        Ok(())
    }

    pub async fn volume(&self) -> Result<Volume, Error> {
        Ok(self.status().await?.volume)
    }

    pub async fn set_volume(&self, level: f64, muted: bool) -> Result<(), Error> {
        self.send_request(
            &self.platform,
            receiver::Receiver::set_volume_request(level, muted),
        )
        .await?;
        Ok(())
    }

    pub async fn status(&self) -> Result<Status, Error> {
        let response = self
            .send_request(&self.platform, receiver::Receiver::GetStatus)
            .await?;

        if let Payload::Receiver(payload) = response.payload {
            if let receiver::Receiver::ReceiverStatus(ReceiverStatusResponse { status }) = payload {
                return Ok(status);
            }
        }

        Err(Error::NoResponse)
    }

    pub async fn send<P: Into<Payload>>(&self, app: &Application, payload: P) -> Result<(), Error> {
        let payload: Payload = payload.into();
        let namespace = payload.namespace();
        if !app.namespaces.contains(&namespace) {
            return Err(Error::UnsupportedNamespace);
        }

        let client = match self.client().await {
            Some(client) => client,
            None => {
                return Err(Error::NoConnection);
            }
        };

        client.send(app.transport_id.clone(), payload, None).await?;
        Ok(())
    }

    pub async fn send_request<P: Into<Payload>>(
        &self,
        app: &Application,
        payload: P,
    ) -> Result<Response, Error> {
        let payload: Payload = payload.into();
        let namespace = payload.namespace();
        if !app.namespaces.contains(&namespace) {
            return Err(Error::UnsupportedNamespace);
        }

        let client = match self.client().await {
            Some(client) => client,
            None => {
                return Err(Error::NoConnection);
            }
        };

        let (response_tx, response_rx) = async_channel::bounded(1);

        // Each request message gets a unique requestId
        let request_id = {
            let mut id = self.request_id.lock().await;
            *id += 1;
            *id
        };

        // Store request to be able to assign the response in `process_message()`
        let mut requests = self.requests.lock().await;
        requests.insert(request_id, response_tx);
        drop(requests);

        client
            .send(app.transport_id.clone(), payload, Some(request_id))
            .await?;

        // Wait up to 10 seconds before giving up the request
        let res = response_rx.recv().timeout(Duration::from_secs(10)).await;
        match res {
            Some(response) => Ok(response?),
            None => {
                let mut requests = self.requests.lock().await;
                requests.remove(&request_id);
                Err(Error::ResponseTimeout)
            }
        }
    }

    async fn process_response(&self, response: Response) -> Result<(), Error> {
        // Check if this payload is a response to a sent request
        if let Some(request_id) = response.request_id {
            if request_id != 0 {
                match self.requests.lock().await.remove(&request_id) {
                    Some(sender) => sender.send(response.clone()).await?,
                    None => debug!("Ignore payload with unknown requestId"),
                }
            }
        }

        match &response.payload {
            Payload::Heartbeat(heartbeat_message) => {
                match heartbeat_message {
                    Heartbeat::Ping => {
                        self.send(&self.platform, Heartbeat::Pong).await?;
                    }
                    _ => (),
                };
            }
            _ => (),
        }

        Ok(())
    }

    async fn client(&self) -> Option<Client> {
        self.client.lock().await.clone()
    }
}

impl Default for Receiver {
    fn default() -> Self {
        Self::new()
    }
}
