//! Configuration for connecting to aisstream.io.
//!
//! [`AisStreamConfig`] can be built in code or loaded from a JSON file. Field names in the file
//! use snake_case; the wire format sent over the WebSocket uses PascalCase and lives in
//! [`crate::websocket::model::SubscriptionMessage`].

use std::{
    fs::File,
    io::{BufReader, Error},
    path::Path,
};

use serde::{Deserialize, Serialize};
use tracing::{debug, info};

/// Everything needed to open a stream: endpoint, area of interest, credentials, and optional filters.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AisStreamConfig {
    /// WebSocket base URL, e.g. `wss://stream.aisstream.io`.
    /// The client appends `/v0/stream` automatically.
    pub base_url: String,
    /// Geographic bounding boxes as `[[[lat, lon], [lat, lon]], ...]`.
    pub bounding_boxes: Vec<Vec<[f64; 2]>>,
    pub credentials: AisStreamCredentials,
    /// Only receive messages from these MMSI numbers (max 50).
    pub filter_ship_mmsi: Option<Vec<String>>,
    /// Only receive these AIS message types.
    pub filter_message_type: Option<Vec<MessageType>>,
}

/// API credentials for aisstream.io.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AisStreamCredentials {
    pub api_key: String,
    #[serde(default)]
    pub api_secret: Option<String>,
}

/// AIS message type names as used by the aisstream.io API.
///
/// See the [API documentation](https://aisstream.io/documentation) for the full list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum MessageType {
    PositionReport,
    AddressedSafetyMessage,
    AddressedBinaryMessage,
    AidsToNavigationReport,
    AssignedModeCommand,
    BaseStationReport,
    BinaryAcknowledge,
    BinaryBroadcastMessage,
    ChannelManagement,
    CoordinatedUTCInquiry,
    DataLinkManagementMessage,
    DataLinkManagementMessageData,
    ExtendedClassBPositionReport,
    GroupAssignmentCommand,
    GnssBroadcastBinaryMessage,
    Interrogation,
    LongRangeAisBroadcastMessage,
    MultiSlotBinaryMessage,
    SafetyBroadcastMessage,
    ShipStaticData,
    SingleSlotBinaryMessage,
    StandardClassBPositionReport,
    StandardSearchAndRescueAircraftReport,
    StaticDataReport,
    UnknownMessage,
}

impl AisStreamConfig {
    /// Build a config in code. Pass `None` for filters you don't need.
    pub fn new(
        base_url: String,
        bounding_boxes: Vec<Vec<[f64; 2]>>,
        credentials: AisStreamCredentials,
        filter_ship_mmsi: Option<Vec<String>>,
        filter_message_type: Option<Vec<MessageType>>,
    ) -> Self {
        Self {
            base_url,
            bounding_boxes,
            credentials,
            filter_ship_mmsi,
            filter_message_type,
        }
    }

    /// Load config from a JSON file (snake_case field names).
    pub fn from_file(path: &Path) -> Result<Self, Error> {
        info!(path = %path.display(), "loading aisstream config from file");
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: Self = serde_json::from_reader(reader)?;
        debug!(
            base_url = %config.base_url,
            bounding_boxes = config.bounding_boxes.len(),
            "config loaded successfully"
        );
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_full_config() {
        let json = r#"{
            "base_url": "wss://stream.aisstream.io/v0/stream",
            "bounding_boxes": [
                [[25.835302, -80.207729], [25.602700, -79.879297]],
                [[33.772292, -118.356139], [33.673490, -118.095731]]
            ],
            "credentials": {
                "api_key": "test-api-key",
                "api_secret": "test-secret"
            },
            "filter_ship_mmsi": ["368207620", "367719770"],
            "filter_message_type": ["PositionReport", "ShipStaticData"]
        }"#;

        let config = serde_json::from_str::<AisStreamConfig>(json).expect("valid config json");

        assert_eq!(config.base_url, "wss://stream.aisstream.io/v0/stream");
        assert_eq!(config.bounding_boxes.len(), 2);
        assert_eq!(
            config.bounding_boxes[0],
            vec![[25.835302, -80.207729], [25.602700, -79.879297]]
        );
        assert_eq!(
            config.bounding_boxes[1],
            vec![[33.772292, -118.356139], [33.673490, -118.095731]]
        );
        assert_eq!(config.credentials.api_key, "test-api-key");
        assert_eq!(
            config.credentials.api_secret.as_deref(),
            Some("test-secret")
        );
        assert_eq!(
            config.filter_ship_mmsi,
            Some(vec!["368207620".to_string(), "367719770".to_string()])
        );
        assert_eq!(
            config.filter_message_type,
            Some(vec![
                MessageType::PositionReport,
                MessageType::ShipStaticData
            ])
        );
    }

    #[test]
    fn deserializes_minimal_config_with_defaults() {
        let json = r#"{
            "base_url": "wss://stream.aisstream.io/v0/stream",
            "bounding_boxes": [[[0.0, 0.0], [1.0, 1.0]]],
            "credentials": { "api_key": "key" }
        }"#;

        let config = serde_json::from_str::<AisStreamConfig>(json).expect("valid config json");

        assert!(config.filter_ship_mmsi.is_none());
        assert!(config.filter_message_type.is_none());
        assert!(config.credentials.api_secret.is_none());
    }

    #[test]
    fn rejects_invalid_json() {
        let err = serde_json::from_str::<AisStreamConfig>("{ not json }").unwrap_err();
        assert!(err.is_syntax());
    }

    #[test]
    fn rejects_missing_required_fields() {
        let json = r#"{
            "base_url": "wss://stream.aisstream.io/v0/stream",
            "bounding_boxes": [[[0.0, 0.0], [1.0, 1.0]]]
        }"#;

        let err = serde_json::from_str::<AisStreamConfig>(json).unwrap_err();
        assert!(err.is_data());
    }
}
