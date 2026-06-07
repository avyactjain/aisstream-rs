//! Async WebSocket client for aisstream.io.
//!
//! [`AisWebsocketClient`] connects, sends a [`SubscriptionMessage`], and yields [`AisMessage`]
//! values on a stream. If the connection drops, it reconnects using the configured
//! [`AISStreamReconnectStrategy`].

use std::pin::Pin;

use futures_util::{SinkExt, Stream, StreamExt};
use tokio_stream::wrappers::ReceiverStream;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info, trace, warn};
use url::Url;

use crate::{
    config::AisStreamConfig,
    websocket::{
        model::{AisMessage, SubscriptionMessage},
        reconnect_strategy::{AISStreamReconnectStrategy, ExponentialBackoff},
    },
};

use tokio::sync::mpsc;

const AIS_STREAM_PATH: &str = "/v0/stream";

/// A stream of AIS messages from the websocket. Stays open across reconnects.
pub type AisMessageStream = Pin<Box<dyn Stream<Item = anyhow::Result<AisMessage>> + Send>>;

/// Client for streaming AIS data from aisstream.io.
///
/// Created with [`Self::new`] (exponential backoff reconnect) or [`Self::with_strategy`] for a
/// custom reconnect policy.
#[derive(Clone)]
pub struct AisWebsocketClient<R = ExponentialBackoff> {
    config: AisStreamConfig,
    reconnect_strategy: R,
}

impl AisWebsocketClient<ExponentialBackoff> {
    /// Connect with the default exponential backoff reconnect strategy.
    pub fn new(config: AisStreamConfig) -> Self {
        debug!(
            base_url = %config.base_url,
            bounding_boxes = config.bounding_boxes.len(),
            mmsi_filters = config.filter_ship_mmsi.as_ref().map(Vec::len).unwrap_or(0),
            message_type_filters = config.filter_message_type.as_ref().map(Vec::len).unwrap_or(0),
            "created websocket client with default exponential backoff"
        );
        Self {
            config,
            reconnect_strategy: ExponentialBackoff::default(),
        }
    }
}

impl<R> AisWebsocketClient<R>
where
    R: AISStreamReconnectStrategy + Clone + Send + 'static,
{
    /// Connect with a custom reconnect strategy.
    pub fn with_strategy(config: AisStreamConfig, reconnect_strategy: R) -> Self {
        debug!(
            base_url = %config.base_url,
            "created websocket client with custom reconnect strategy"
        );
        Self {
            config,
            reconnect_strategy,
        }
    }

    /// Open the stream. Spawns a background task that keeps the connection alive and reconnects
    /// on failure. Each item is a parsed [`AisMessage`] or a deserialization error.
    pub async fn stream(&mut self) -> anyhow::Result<AisMessageStream> {
        let (tx, rx) = mpsc::channel(1024);

        let mut url = Url::parse(&self.config.base_url)?;
        url.set_path(AIS_STREAM_PATH);

        let mut reconnect_strategy = self.reconnect_strategy.clone();

        let subscription = SubscriptionMessage {
            api_key: self.config.credentials.api_key.clone(),
            bounding_boxes: self.config.bounding_boxes.clone(),
            filters_ship_mmsi: self.config.filter_ship_mmsi.clone().unwrap_or_default(),
            filter_message_types: self.config.filter_message_type.clone().unwrap_or_default(),
        };

        let payload = serde_json::to_string(&subscription)?;
        let stream_url = url.to_string();

        info!(url = %stream_url, "starting aisstream websocket background task");

        tokio::spawn(async move {
            loop {
                debug!(url = %stream_url, "connecting to aisstream");
                match tokio_tungstenite::connect_async(&stream_url).await {
                    Ok((ws_stream, _)) => {
                        info!(url = %stream_url, "connected to aisstream");
                        reconnect_strategy.reset();

                        let (mut write, mut read) = ws_stream.split();

                        if write.send(Message::Text(payload.clone().into())).await.is_err() {
                            warn!(url = %stream_url, "failed to send subscription message");
                            continue;
                        }

                        debug!(
                            bounding_boxes = subscription.bounding_boxes.len(),
                            mmsi_filters = subscription.filters_ship_mmsi.len(),
                            message_type_filters = subscription.filter_message_types.len(),
                            "subscription message sent"
                        );

                        while let Some(msg) = read.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    if let Err(err) = handle_payload(&text, &tx).await {
                                        warn!(error = %err, "failed to handle text frame");
                                        let _ = tx.send(Err(err)).await;
                                    }
                                }

                                Ok(Message::Binary(binary)) => {
                                    let text = match std::str::from_utf8(&binary) {
                                        Ok(s) => s,
                                        Err(err) => {
                                            warn!(error = %err, "binary frame is not valid utf-8");
                                            let _ = tx
                                                .send(Err(anyhow::anyhow!(
                                                    "invalid utf-8 in binary frame: {err}"
                                                )))
                                                .await;
                                            continue;
                                        }
                                    };
                                    if let Err(err) = handle_payload(text, &tx).await {
                                        warn!(error = %err, "failed to handle binary frame");
                                        let _ = tx.send(Err(err)).await;
                                    }
                                }

                                Ok(Message::Ping(_) | Message::Pong(_) | Message::Frame(_)) => {
                                    trace!("received websocket control frame");
                                }

                                Ok(Message::Close(frame)) => {
                                    info!(?frame, "websocket closed by server");
                                    break;
                                }

                                Err(err) => {
                                    error!(error = %err, "websocket read error");
                                    break;
                                }
                            }
                        }

                        debug!("websocket read loop ended, will reconnect");
                    }

                    Err(err) => {
                        warn!(error = %err, url = %stream_url, "websocket connect failed");
                    }
                }

                let delay = reconnect_strategy.next_delay();
                info!(
                    delay_secs = delay.as_secs_f64(),
                    "waiting before reconnect attempt"
                );
                tokio::time::sleep(delay).await;
            }
        });

        debug!("ais message stream channel created");
        Ok(Box::pin(ReceiverStream::new(rx)))
    }
}

async fn handle_payload(
    text: &str,
    tx: &mpsc::Sender<anyhow::Result<AisMessage>>,
) -> anyhow::Result<()> {
    let message: AisMessage = serde_json::from_str(text).map_err(|err| {
        debug!(error = %err, payload_len = text.len(), "failed to deserialize ais message");
        err
    })?;

    trace!(
        message_type = ?message.message_type,
        mmsi = message.metadata.mmsi,
        "received ais message"
    );

    if tx.send(Ok(message)).await.is_err() {
        debug!("stream receiver dropped, stopping message forwarding");
        return Err(anyhow::anyhow!("stream receiver dropped"));
    }

    Ok(())
}
