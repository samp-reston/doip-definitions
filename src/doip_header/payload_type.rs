use crate::{
    definitions::{
        DOIP_ALIVE_CHECK_REQUEST, DOIP_ALIVE_CHECK_RESPONSE, DOIP_DIAGNOSTIC_MESSAGE,
        DOIP_DIAGNOSTIC_MESSAGE_ACK, DOIP_DIAGNOSTIC_MESSAGE_NACK, DOIP_ENTITY_STATUS_REQUEST,
        DOIP_ENTITY_STATUS_RESPONSE, DOIP_GENERIC_NACK, DOIP_POWER_INFORMATION_REQUEST,
        DOIP_POWER_INFORMATION_RESPONSE, DOIP_ROUTING_ACTIVATION_REQUEST,
        DOIP_ROUTING_ACTIVATION_RESPONSE, DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,
        DOIP_VEHICLE_IDENTIFICATION_REQ, DOIP_VEHICLE_IDENTIFICATION_REQ_EID,
        DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,
    },
    error::Error,
};

/// Defines the variants of payloads available to `DoIP`.
///
/// `PayloadType` values map to the `u16` representing the bytes it makes up
/// within the `DoIP` packet.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(u16)]
pub enum PayloadType {
    /// Generic Negative Acknowledge
    GenericNack = DOIP_GENERIC_NACK,

    /// Vehicle Identification Request
    VehicleIdentificationRequest = DOIP_VEHICLE_IDENTIFICATION_REQ,

    /// Vehicle Identification Request by EID
    VehicleIdentificationRequestEid = DOIP_VEHICLE_IDENTIFICATION_REQ_EID,

    /// Vehicle Identification Request by VIN
    VehicleIdentificationRequestVin = DOIP_VEHICLE_IDENTIFICATION_REQ_VIN,

    /// Vehicle Announcement Message
    VehicleAnnouncementMessage = DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE,

    /// Routing Activation Request
    RoutingActivationRequest = DOIP_ROUTING_ACTIVATION_REQUEST,

    /// Routing Activation Response
    RoutingActivationResponse = DOIP_ROUTING_ACTIVATION_RESPONSE,

    /// Alive Check Request
    AliveCheckRequest = DOIP_ALIVE_CHECK_REQUEST,

    /// Alive Check Response
    AliveCheckResponse = DOIP_ALIVE_CHECK_RESPONSE,

    /// Entity Status Request
    EntityStatusRequest = DOIP_ENTITY_STATUS_REQUEST,

    /// Entity Status Response
    EntityStatusResponse = DOIP_ENTITY_STATUS_RESPONSE,

    /// Power Information Request
    PowerInformationRequest = DOIP_POWER_INFORMATION_REQUEST,

    /// Power Information Response
    PowerInformationResponse = DOIP_POWER_INFORMATION_RESPONSE,

    /// Diagnostic Message
    DiagnosticMessage = DOIP_DIAGNOSTIC_MESSAGE,

    /// Diagnostic Message Acknowledgement
    DiagnosticMessageAck = DOIP_DIAGNOSTIC_MESSAGE_ACK,

    /// Diagnostic Message Negative Acknowledgement
    DiagnosticMessageNack = DOIP_DIAGNOSTIC_MESSAGE_NACK,
}

impl TryFrom<&[u8]> for PayloadType {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let val = u16::from_be_bytes([value[0], value[1]]);

        match val {
            x if x == PayloadType::GenericNack as u16 => Ok(PayloadType::GenericNack),
            x if x == PayloadType::VehicleIdentificationRequest as u16 => {
                Ok(PayloadType::VehicleIdentificationRequest)
            }
            x if x == PayloadType::VehicleIdentificationRequestEid as u16 => {
                Ok(PayloadType::VehicleIdentificationRequestEid)
            }
            x if x == PayloadType::VehicleIdentificationRequestVin as u16 => {
                Ok(PayloadType::VehicleIdentificationRequestVin)
            }
            x if x == PayloadType::VehicleAnnouncementMessage as u16 => {
                Ok(PayloadType::VehicleAnnouncementMessage)
            }
            x if x == PayloadType::RoutingActivationRequest as u16 => {
                Ok(PayloadType::RoutingActivationRequest)
            }
            x if x == PayloadType::RoutingActivationResponse as u16 => {
                Ok(PayloadType::RoutingActivationResponse)
            }
            x if x == PayloadType::AliveCheckRequest as u16 => Ok(PayloadType::AliveCheckRequest),
            x if x == PayloadType::AliveCheckResponse as u16 => Ok(PayloadType::AliveCheckResponse),
            x if x == PayloadType::EntityStatusRequest as u16 => {
                Ok(PayloadType::EntityStatusRequest)
            }
            x if x == PayloadType::EntityStatusResponse as u16 => {
                Ok(PayloadType::EntityStatusResponse)
            }
            x if x == PayloadType::PowerInformationRequest as u16 => {
                Ok(PayloadType::PowerInformationRequest)
            }
            x if x == PayloadType::PowerInformationResponse as u16 => {
                Ok(PayloadType::PowerInformationResponse)
            }
            x if x == PayloadType::DiagnosticMessage as u16 => Ok(PayloadType::DiagnosticMessage),
            x if x == PayloadType::DiagnosticMessageAck as u16 => {
                Ok(PayloadType::DiagnosticMessageAck)
            }
            x if x == PayloadType::DiagnosticMessageNack as u16 => {
                Ok(PayloadType::DiagnosticMessageNack)
            }
            _ => Err(Error::InvalidPayloadType {
                value: [value[0], value[1]],
            }),
        }
    }
}

impl From<PayloadType> for [u8; 2] {
    fn from(value: PayloadType) -> Self {
        (value as u16).to_be_bytes()
    }
}
