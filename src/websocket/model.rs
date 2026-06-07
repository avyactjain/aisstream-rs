//! Serde types for the aisstream.io WebSocket API.
//!
//! There are three layers you'll care about:
//!
//! - [`SubscriptionMessage`] — what you send when connecting
//! - [`AisMessage`] — the envelope for each event (`Message`, `MessageType`, `MetaData`)
//! - [`AisStreamMessage`] — the tagged union inside `Message`, one variant per AIS message type
//!
//! The structs below `AisStreamMessage` mirror the
//! [API message models](https://aisstream.io/documentation#API-Message-Models) field-for-field.

use serde::{Deserialize, Serialize};

use crate::config::MessageType;

/// Subscription payload sent immediately after the WebSocket connects.
///
/// Must be sent within 3 seconds of connecting or the server closes the connection.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct SubscriptionMessage {
    #[serde(rename = "APIKey")]
    pub api_key: String,
    #[serde(rename = "BoundingBoxes")]
    pub bounding_boxes: Vec<Vec<[f64; 2]>>,
    #[serde(
        default,
        rename = "FiltersShipMMSI",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filters_ship_mmsi: Vec<String>,
    #[serde(
        default,
        rename = "FilterMessageTypes",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub filter_message_types: Vec<MessageType>,
}

/// Extra context about a message that isn't part of the raw AIS payload — ship name, MMSI, etc.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AisMessageMetadata {
    #[serde(rename = "MMSI")]
    pub mmsi: i64,
    #[serde(rename = "ShipName")]
    pub ship_name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub time_utc: String,
}

/// A single event from the aisstream.io WebSocket.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AisMessage {
    #[serde(rename = "Message")]
    pub message: AisStreamMessage,
    #[serde(rename = "MessageType")]
    pub message_type: MessageType,
    #[serde(rename = "MetaData")]
    pub metadata: AisMessageMetadata,
}

/// The AIS payload inside [`AisMessage::message`], tagged by message type name.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum AisStreamMessage {
    BaseStationReport(BaseStationReport),
    MultiSlotBinaryMessage(MultiSlotBinaryMessage),
    ExtendedClassBPositionReport(ExtendedClassBPositionReport),
    SafetyBroadcastMessage(SafetyBroadcastMessage),
    PositionReport(PositionReport),
    ShipStaticData(ShipStaticData),
    StandardClassBPositionReport(StandardClassBPositionReport),
    StandardSearchAndRescueAircraftReport(StandardSearchAndRescueAircraftReport),
    StaticDataReport(StaticDataReport),
    SingleSlotBinaryMessage(SingleSlotBinaryMessage),
    Interrogation(Interrogation),
    LongRangeAisBroadcastMessage(LongRangeAisBroadcastMessage),
    GnssBroadcastBinaryMessage(GnssBroadcastBinaryMessage),
    DataLinkManagementMessage(DataLinkManagementMessage),
    DataLinkManagementMessageData(DataLinkManagementMessageData),
    AddressedSafetyMessage(AddressedSafetyMessage),
    AddressedBinaryMessage(AddressedBinaryMessage),
    CoordinatedUTCInquiry(CoordinatedUTCInquiry),
    BinaryAcknowledge(BinaryAcknowledge),
    BinaryBroadcastMessage(BinaryBroadcastMessage),
    ChannelManagement(ChannelManagement),
    AssignedModeCommand(AssignedModeCommand),
    AidsToNavigationReport(AidsToNavigationReport),
    GroupAssignmentCommand(GroupAssignmentCommand),
    UnknownMessage(UnknownMessage),
}

/// Application identifier used in binary AIS messages.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ApplicationId {
    pub designated_area_code: i64,
    pub function_identifier: i64,
    pub valid: bool,
}

/// Vessel dimensions: distances from the reference point to bow, stern, port, and starboard.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Dimension {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
}

/// Estimated time of arrival.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Eta {
    pub month: i64,
    pub day: i64,
    pub hour: i64,
    pub minute: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DataLinkManagementMessageDataSlot {
    pub valid: bool,
    pub offset: i64,
    #[serde(rename = "integerOfSlots")]
    pub integer_of_slots: i64,
    pub time_out: i64,
    pub increment: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct DataLinkManagementMessageData {
    #[serde(rename = "0")]
    pub slot_0: DataLinkManagementMessageDataSlot,
    #[serde(rename = "1")]
    pub slot_1: DataLinkManagementMessageDataSlot,
    #[serde(rename = "2")]
    pub slot_2: DataLinkManagementMessageDataSlot,
    #[serde(rename = "3")]
    pub slot_3: DataLinkManagementMessageDataSlot,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BinaryAcknowledgeDestination {
    pub valid: bool,
    #[serde(rename = "DestinationID")]
    pub destination_id: i64,
    pub sequenceinteger: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct BinaryAcknowledgeDestinations {
    #[serde(rename = "0")]
    pub slot_0: BinaryAcknowledgeDestination,
    #[serde(rename = "1")]
    pub slot_1: BinaryAcknowledgeDestination,
    #[serde(rename = "2")]
    pub slot_2: BinaryAcknowledgeDestination,
    #[serde(rename = "3")]
    pub slot_3: BinaryAcknowledgeDestination,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AssignedModeCommandEntry {
    pub valid: bool,
    #[serde(rename = "DestinationID")]
    pub destination_id: i64,
    pub offset: i64,
    pub increment: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub struct AssignedModeCommandCommands {
    #[serde(rename = "0")]
    pub slot_0: AssignedModeCommandEntry,
    #[serde(rename = "1")]
    pub slot_1: AssignedModeCommandEntry,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InterrogationStation1Msg1 {
    pub valid: bool,
    #[serde(rename = "StationID")]
    pub station_id: i64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub slot_offset: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InterrogationStation1Msg2 {
    pub valid: bool,
    pub spare: i64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub slot_offset: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct InterrogationStation2 {
    pub valid: bool,
    pub spare1: i64,
    #[serde(rename = "StationID")]
    pub station_id: i64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub slot_offset: i64,
    pub spare2: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelManagementArea {
    pub longitude1: f64,
    pub latitude1: f64,
    pub longitude2: f64,
    pub latitude2: f64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelManagementUnicast {
    pub address_station1: i64,
    pub spare2: i64,
    pub address_station2: i64,
    pub spare3: i64,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StaticDataReportA {
    pub valid: bool,
    pub name: String,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StaticDataReportB {
    pub valid: bool,
    pub ship_type: i64,
    #[serde(rename = "VendorIDName")]
    pub vendor_id_name: String,
    #[serde(rename = "VenderIDModel")]
    pub vender_id_model: i64,
    #[serde(rename = "VenderIDSerial")]
    pub vender_id_serial: i64,
    pub call_sign: String,
    pub dimension: Dimension,
    pub fix_type: i64,
    pub spare: i64,
}

/// AIS message type 4 — base station position and UTC time.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BaseStationReport {
    pub communication_state: i64,
    pub fix_type: i64,
    pub latitude: f64,
    pub long_range_enable: bool,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub position_accuracy: bool,
    pub raim: bool,
    pub repeat_indicator: i64,
    pub spare: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub utc_day: i64,
    pub utc_hour: i64,
    pub utc_minute: i64,
    pub utc_month: i64,
    pub utc_second: i64,
    pub utc_year: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MultiSlotBinaryMessage {
    #[serde(rename = "ApplicationID")]
    pub application_id: ApplicationId,
    #[serde(rename = "ApplicationIDValid")]
    pub application_id_valid: bool,
    pub communication_state: i64,
    pub communication_state_is_itdma: bool,
    #[serde(rename = "DestinationID")]
    pub destination_id: i64,
    #[serde(rename = "DestinationIDValid")]
    pub destination_id_valid: bool,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub payload: String,
    pub repeat_indicator: i64,
    pub spare1: i64,
    pub spare2: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExtendedClassBPositionReport {
    pub assigned_mode: bool,
    pub cog: f64,
    pub dimension: Dimension,
    pub dte: bool,
    pub fix_type: i64,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub name: String,
    pub position_accuracy: bool,
    pub raim: bool,
    pub repeat_indicator: i64,
    pub sog: f64,
    pub spare1: i64,
    pub spare2: i64,
    pub spare3: i64,
    pub timestamp: i64,
    pub true_heading: i64,
    pub r#type: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SafetyBroadcastMessage {
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare: i64,
    pub text: String,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

/// AIS message type 1/2/3 — vessel position, course, and speed.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PositionReport {
    pub cog: f64,
    pub communication_state: i64,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub navigational_status: i64,
    pub position_accuracy: bool,
    pub raim: bool,
    pub rate_of_turn: i64,
    pub repeat_indicator: i64,
    pub sog: f64,
    pub spare: i64,
    pub special_manoeuvre_indicator: i64,
    pub timestamp: i64,
    pub true_heading: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

/// AIS message type 5 — static vessel data (name, dimensions, destination, etc.).
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ShipStaticData {
    pub ais_version: i64,
    pub call_sign: String,
    pub destination: String,
    pub dimension: Dimension,
    pub dte: bool,
    pub eta: Eta,
    pub fix_type: i64,
    pub imo_number: i64,
    pub maximum_static_draught: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub name: String,
    pub repeat_indicator: i64,
    pub spare: bool,
    pub r#type: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StandardClassBPositionReport {
    pub assigned_mode: bool,
    pub class_b_band: bool,
    pub class_b_display: bool,
    pub class_b_dsc: bool,
    pub class_b_msg22: bool,
    pub class_b_unit: bool,
    pub cog: f64,
    pub communication_state: i64,
    pub communication_state_is_itdma: bool,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub position_accuracy: bool,
    pub raim: bool,
    pub repeat_indicator: i64,
    pub sog: f64,
    pub spare1: i64,
    pub spare2: i64,
    pub timestamp: i64,
    pub true_heading: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StandardSearchAndRescueAircraftReport {
    pub alt_from_baro: bool,
    pub altitude: i64,
    pub assigned_mode: bool,
    pub cog: f64,
    pub communication_state: i64,
    pub communication_state_is_itdma: bool,
    pub dte: bool,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub position_accuracy: bool,
    pub raim: bool,
    pub repeat_indicator: i64,
    pub sog: f64,
    pub spare1: i64,
    pub spare2: i64,
    pub timestamp: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StaticDataReport {
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub part_number: bool,
    pub repeat_indicator: i64,
    pub report_a: StaticDataReportA,
    pub report_b: StaticDataReportB,
    pub reserved: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SingleSlotBinaryMessage {
    #[serde(rename = "ApplicationID")]
    pub application_id: ApplicationId,
    #[serde(rename = "ApplicationIDValid")]
    pub application_id_valid: bool,
    #[serde(rename = "DestinationID")]
    pub destination_id: i64,
    #[serde(rename = "DestinationIDValid")]
    pub destination_id_valid: bool,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub payload: String,
    pub repeat_indicator: i64,
    pub spare: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Interrogation {
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare: i64,
    pub station1_msg1: InterrogationStation1Msg1,
    pub station1_msg2: InterrogationStation1Msg2,
    pub station2: InterrogationStation2,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LongRangeAisBroadcastMessage {
    pub cog: f64,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub navigational_status: i64,
    pub position_accuracy: bool,
    pub position_latency: bool,
    pub raim: bool,
    pub repeat_indicator: i64,
    pub sog: f64,
    pub spare: bool,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GnssBroadcastBinaryMessage {
    pub data: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare1: i64,
    pub spare2: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct DataLinkManagementMessage {
    pub data: DataLinkManagementMessageData,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddressedSafetyMessage {
    #[serde(rename = "DestinationID")]
    pub destination_id: i64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub retransmission: bool,
    pub sequenceinteger: i64,
    pub spare: bool,
    pub text: String,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AddressedBinaryMessage {
    #[serde(rename = "ApplicationID")]
    pub application_id: ApplicationId,
    pub binary_data: String,
    #[serde(rename = "DestinationID")]
    pub destination_id: i64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub retransmission: bool,
    pub sequenceinteger: i64,
    pub spare: bool,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CoordinatedUTCInquiry {
    #[serde(rename = "DestinationID")]
    pub destination_id: i64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare1: i64,
    pub spare2: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BinaryAcknowledge {
    pub destinations: BinaryAcknowledgeDestinations,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BinaryBroadcastMessage {
    #[serde(rename = "ApplicationID")]
    pub application_id: ApplicationId,
    pub binary_data: String,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ChannelManagement {
    pub area: ChannelManagementArea,
    pub bw_a: bool,
    pub bw_b: bool,
    pub channel_a: i64,
    pub channel_b: i64,
    pub is_addressed: bool,
    pub low_power: bool,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare1: i64,
    pub spare4: i64,
    pub transitional_zone_size: i64,
    pub tx_rx_mode: i64,
    pub unicast: ChannelManagementUnicast,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AssignedModeCommand {
    pub commands: AssignedModeCommandCommands,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub repeat_indicator: i64,
    pub spare: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AidsToNavigationReport {
    pub assigned_mode: bool,
    pub ato_n: i64,
    pub dimension: Dimension,
    #[serde(rename = "Fixtype")]
    pub fixtype: i64,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub name: String,
    pub name_extension: String,
    pub off_position: bool,
    pub position_accuracy: bool,
    pub raim: bool,
    pub repeat_indicator: i64,
    pub spare: bool,
    pub timestamp: i64,
    pub r#type: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
    pub virtual_ato_n: bool,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct GroupAssignmentCommand {
    pub latitude1: f64,
    pub latitude2: f64,
    pub longitude1: f64,
    pub longitude2: f64,
    #[serde(rename = "MessageID")]
    pub message_id: i64,
    pub quiet_time: i64,
    pub repeat_indicator: i64,
    pub reporting_interval: i64,
    pub ship_type: i64,
    pub spare1: i64,
    pub spare2: i64,
    pub spare3: i64,
    pub station_type: i64,
    pub tx_rx_mode: i64,
    #[serde(rename = "UserID")]
    pub user_id: i64,
    pub valid: bool,
}

/// Returned when the server sends a message type this client doesn't recognise.
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UnknownMessage {
    pub error: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn deserialize<T: for<'de> Deserialize<'de>>(json: &str) -> T {
        serde_json::from_str(json).expect("valid message json")
    }

    #[test]
    fn deserializes_ais_message() {
        let msg: AisMessage = deserialize(
            r#"{
                "Message": {
                    "PositionReport": {
                        "Cog": 308,
                        "CommunicationState": 81982,
                        "Latitude": 66.02695,
                        "Longitude": 12.253821666666665,
                        "MessageID": 1,
                        "NavigationalStatus": 15,
                        "PositionAccuracy": true,
                        "Raim": false,
                        "RateOfTurn": 4,
                        "RepeatIndicator": 0,
                        "Sog": 0,
                        "Spare": 0,
                        "SpecialManoeuvreIndicator": 0,
                        "Timestamp": 31,
                        "TrueHeading": 235,
                        "UserID": 259000420,
                        "Valid": true
                    }
                },
                "MessageType": "PositionReport",
                "MetaData": {
                    "MMSI": 259000420,
                    "ShipName": "AUGUSTSON",
                    "latitude": 66.02695,
                    "longitude": 12.253821666666665,
                    "time_utc": "2022-12-29 18:22:32.318353 +0000 UTC"
                }
            }"#,
        );
        assert_eq!(msg.message_type, MessageType::PositionReport);
        assert_eq!(msg.metadata.mmsi, 259000420);
        assert_eq!(msg.metadata.ship_name, "AUGUSTSON");
        assert_eq!(msg.metadata.latitude, 66.02695);
        match msg.message {
            AisStreamMessage::PositionReport(report) => {
                assert_eq!(report.user_id, 259000420);
                assert_eq!(report.cog, 308.0);
                assert_eq!(report.true_heading, 235);
            }
            _ => panic!("expected PositionReport variant"),
        }
    }

    #[test]
    fn deserializes_subscription_message() {
        let msg: SubscriptionMessage = deserialize(
            r#"{
                "APIKey": "test-api-key",
                "BoundingBoxes": [
                    [[25.835302, -80.207729], [25.602700, -79.879297]],
                    [[33.772292, -118.356139], [33.673490, -118.095731]]
                ],
                "FiltersShipMMSI": ["368207620", "367719770", "211476060"],
                "FilterMessageTypes": ["PositionReport"]
            }"#,
        );
        assert_eq!(msg.api_key, "test-api-key");
        assert_eq!(msg.bounding_boxes.len(), 2);
        assert_eq!(
            msg.bounding_boxes[0],
            [[25.835302, -80.207729], [25.602700, -79.879297]]
        );
        assert_eq!(
            msg.filters_ship_mmsi,
            vec!["368207620", "367719770", "211476060"]
        );
        assert_eq!(msg.filter_message_types, vec![MessageType::PositionReport]);
    }

    #[test]
    fn serializes_subscription_message_without_optional_fields() {
        let msg = SubscriptionMessage {
            api_key: "key".to_string(),
            bounding_boxes: vec![vec![[0.0, 0.0], [1.0, 1.0]]],
            filters_ship_mmsi: vec![],
            filter_message_types: vec![],
        };
        let json = serde_json::to_string(&msg).expect("serialize subscription");
        assert!(json.contains(r#""APIKey":"key""#));
        assert!(json.contains(r#""BoundingBoxes":"#));
        assert!(!json.contains("FiltersShipMMSI"));
        assert!(!json.contains("FilterMessageTypes"));
    }

    #[test]
    fn deserializes_base_station_report() {
        let msg: BaseStationReport = deserialize(
            r#"{
                "CommunicationState": 20180,
                "FixType": 15,
                "Latitude": 43.49155666666666,
                "LongRangeEnable": false,
                "Longitude": -5.941905,
                "MessageID": 4,
                "PositionAccuracy": false,
                "Raim": true,
                "RepeatIndicator": 0,
                "Spare": 0,
                "UserID": 2241118,
                "UtcDay": 9,
                "UtcHour": 7,
                "UtcMinute": 53,
                "UtcMonth": 9,
                "UtcSecond": 30,
                "UtcYear": 2022,
                "Valid": true
            }"#,
        );
        assert_eq!(msg.message_id, 4);
        assert_eq!(msg.user_id, 2241118);
        assert!(msg.valid);
    }

    #[test]
    fn deserializes_position_report() {
        let msg: PositionReport = deserialize(
            r#"{
                "Cog": 0,
                "CommunicationState": 59916,
                "Latitude": 51.44458833333333,
                "Longitude": 3.590816666666667,
                "MessageID": 1,
                "NavigationalStatus": 7,
                "PositionAccuracy": true,
                "Raim": true,
                "RateOfTurn": 0,
                "RepeatIndicator": 0,
                "Sog": 0,
                "Spare": 0,
                "SpecialManoeuvreIndicator": 0,
                "Timestamp": 12,
                "TrueHeading": 17,
                "UserID": 245473000,
                "Valid": true
            }"#,
        );
        assert_eq!(msg.user_id, 245473000);
        assert_eq!(msg.true_heading, 17);
    }

    #[test]
    fn deserializes_ship_static_data() {
        let msg: ShipStaticData = deserialize(
            r#"{
                "AisVersion": 2,
                "CallSign": "LBHF",
                "Destination": "COASTGUARD@@@@@@@@H",
                "Dimension": { "A": 20, "B": 27, "C": 7, "D": 7 },
                "Dte": false,
                "Eta": { "Day": 0, "Hour": 0, "Minute": 0, "Month": 0 },
                "FixType": 1,
                "ImoNumber": 9353333,
                "MaximumStaticDraught": 4.5,
                "MessageID": 5,
                "Name": "KV FARM",
                "RepeatIndicator": 0,
                "Spare": false,
                "Type": 55,
                "UserID": 257069200,
                "Valid": true
            }"#,
        );
        assert_eq!(msg.name, "KV FARM");
        assert_eq!(msg.dimension.a, 20);
    }

    #[test]
    fn deserializes_static_data_report() {
        let msg: StaticDataReport = deserialize(
            r#"{
                "MessageID": 24,
                "PartNumber": true,
                "RepeatIndicator": 0,
                "ReportA": { "Name": "", "Valid": false },
                "ReportB": {
                    "CallSign": "LESW",
                    "Dimension": { "A": 12, "B": 3, "C": 3, "D": 2 },
                    "FixType": 0,
                    "ShipType": 37,
                    "Spare": 0,
                    "Valid": true,
                    "VenderIDModel": 1,
                    "VenderIDSerial": 292978,
                    "VendorIDName": "SRT"
                },
                "Reserved": 0,
                "UserID": 257702970,
                "Valid": true
            }"#,
        );
        assert_eq!(msg.report_b.vendor_id_name, "SRT");
        assert_eq!(msg.report_b.vender_id_model, 1);
    }

    #[test]
    fn deserializes_data_link_management_message() {
        let msg: DataLinkManagementMessage = deserialize(
            r#"{
                "Data": {
                    "0": { "Increment": 750, "Offset": 623, "TimeOut": 7, "Valid": false, "integerOfSlots": 1 },
                    "1": { "Increment": 1125, "Offset": 1125, "TimeOut": 7, "Valid": false, "integerOfSlots": 1 },
                    "2": { "Increment": 0, "Offset": 0, "TimeOut": 0, "Valid": false, "integerOfSlots": 0 },
                    "3": { "Increment": 0, "Offset": 0, "TimeOut": 0, "Valid": false, "integerOfSlots": 0 }
                },
                "MessageID": 20,
                "RepeatIndicator": 0,
                "Spare": 0,
                "UserID": 2655069,
                "Valid": true
            }"#,
        );
        assert_eq!(msg.data.slot_0.offset, 623);
        assert_eq!(msg.data.slot_0.integer_of_slots, 1);
    }

    #[test]
    fn deserializes_data_link_management_message_data() {
        let msg: DataLinkManagementMessageData = deserialize(
            r#"{
                "0": { "Increment": 750, "Offset": 623, "TimeOut": 7, "Valid": false, "integerOfSlots": 1 },
                "1": { "Increment": 1125, "Offset": 1125, "TimeOut": 7, "Valid": false, "integerOfSlots": 1 },
                "2": { "Increment": 0, "Offset": 0, "TimeOut": 0, "Valid": false, "integerOfSlots": 0 },
                "3": { "Increment": 0, "Offset": 0, "TimeOut": 0, "Valid": false, "integerOfSlots": 0 }
            }"#,
        );
        assert_eq!(msg.slot_1.increment, 1125);
    }

    #[test]
    fn deserializes_binary_acknowledge() {
        let msg: BinaryAcknowledge = deserialize(
            r#"{
                "Destinations": {
                    "0": { "DestinationID": 992351360, "Sequenceinteger": 0, "Valid": true },
                    "1": { "DestinationID": 0, "Sequenceinteger": 0, "Valid": false },
                    "2": { "DestinationID": 0, "Sequenceinteger": 0, "Valid": false },
                    "3": { "DestinationID": 0, "Sequenceinteger": 0, "Valid": false }
                },
                "MessageID": 7,
                "RepeatIndicator": 0,
                "Spare": 0,
                "UserID": 2320075,
                "Valid": true
            }"#,
        );
        assert_eq!(msg.destinations.slot_0.destination_id, 992351360);
    }

    #[test]
    fn deserializes_aids_to_navigation_report() {
        let msg: AidsToNavigationReport = deserialize(
            r#"{
                "AssignedMode": false,
                "AtoN": 0,
                "Dimension": { "A": 0, "B": 0, "C": 0, "D": 0 },
                "Fixtype": 7,
                "Latitude": 30.099798333333336,
                "Longitude": -90.91296166666666,
                "MessageID": 21,
                "Name": "B                   ",
                "NameExtension": "",
                "OffPosition": false,
                "PositionAccuracy": false,
                "Raim": false,
                "RepeatIndicator": 0,
                "Spare": false,
                "Timestamp": 61,
                "Type": 26,
                "UserID": 993682816,
                "Valid": true,
                "VirtualAtoN": true
            }"#,
        );
        assert_eq!(msg.fixtype, 7);
        assert!(msg.virtual_ato_n);
    }
}
