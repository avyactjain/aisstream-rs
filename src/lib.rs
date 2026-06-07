//! A Rust client for [aisstream.io](https://aisstream.io) — real-time AIS vessel data over WebSockets.
//!
//! Connect, subscribe to a geographic area, and receive strongly typed AIS messages as an async
//! stream. The client handles subscription on connect and reconnects automatically when the
//! connection drops.
//!
//! # Quick start
//!
//! ```no_run
//! use aisstream_rs::{AisStreamConfig, AisStreamCredentials, AisWebsocketClient};
//! use futures_util::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let config = AisStreamConfig::new(
//!         "wss://stream.aisstream.io".into(),
//!         vec![vec![[25.835302, -80.207729], [25.602700, -79.879297]]],
//!         AisStreamCredentials {
//!             api_key: std::env::var("AISSTREAM_API_KEY")?,
//!             api_secret: None,
//!         },
//!         None,
//!         None,
//!     );
//!
//!     let mut client = AisWebsocketClient::new(config);
//!     let mut stream = client.stream().await?;
//!
//!     while let Some(message) = stream.next().await {
//!         println!("{message:?}");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Logging
//!
//! This crate uses [`tracing`] for structured logs. Install a subscriber in your binary to see
//! them — for example with `tracing-subscriber`:
//!
//! ```ignore
//! tracing_subscriber::fmt::init();
//! ```
//!
//! Connection events are logged at `info`, per-message details at `trace`, and errors at
//! `warn` / `error`.
//!
//! # Modules
//!
//! - [`config`] — connection settings loaded from JSON or built in code
//! - [`websocket`] — the streaming client, message types, and reconnect strategies

pub mod config;
pub mod websocket;

pub use config::{AisStreamConfig, AisStreamCredentials, MessageType};
pub use websocket::ais_websocket_client::{AisMessageStream, AisWebsocketClient};
pub use websocket::model::{AisMessage, AisMessageMetadata, AisStreamMessage, SubscriptionMessage};
pub use websocket::reconnect_strategy::{AISStreamReconnectStrategy, ExponentialBackoff};
