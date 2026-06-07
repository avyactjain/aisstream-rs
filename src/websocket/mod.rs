//! WebSocket client and AIS message types for the aisstream.io API.
//!
//! Most callers only need [`AisWebsocketClient`](ais_websocket_client::AisWebsocketClient) and the
//! message types in [`model`].

pub mod ais_websocket_client;
pub mod model;
pub mod reconnect_strategy;
