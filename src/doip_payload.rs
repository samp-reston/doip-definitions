use alive_check_request::AliveCheckRequest;
use alive_check_response::AliveCheckResponse;
use diagnostic_message::DiagnosticMessage;
use diagnostic_message_ack::DiagnosticMessageAck;
use diagnostic_message_nack::DiagnosticMessageNack;
use entity_status_request::EntityStatusRequest;
use entity_status_response::EntityStatusResponse;
use generic_nack::GenericNack;
use power_information_request::PowerInformationRequest;
use power_information_response::PowerInformationResponse;
use routing_activation_request::RoutingActivationRequest;
use routing_activation_response::RoutingActivationResponse;
use vehicle_announcement_message::VehicleAnnouncementMessage;
use vehicle_identification_request::VehicleIdentificationRequest;
use vehicle_identification_request_eid::VehicleIdentificationRequestEid;
use vehicle_identification_request_vin::VehicleIdentificationRequestVin;

pub mod action_code;
pub mod activation_code;
pub mod activation_type;
pub mod diagnostic_ack;
pub mod diagnostic_nack;
pub mod nack_code;
pub mod node_type;
pub mod power_mode;
pub mod sync_status;

pub mod alive_check_request;
pub mod alive_check_response;
pub mod diagnostic_message;
pub mod diagnostic_message_ack;
pub mod diagnostic_message_nack;
pub mod entity_status_request;
pub mod entity_status_response;
pub mod generic_nack;
pub mod power_information_request;
pub mod power_information_response;
pub mod routing_activation_request;
pub mod routing_activation_response;
pub mod vehicle_announcement_message;
pub mod vehicle_identification_request;
pub mod vehicle_identification_request_eid;
pub mod vehicle_identification_request_vin;

/// Implemented across `DoIP` Payload Types for consistent encoding and decoding of buffers.
///
/// `DoipPayload` is implemented for all the `DoIP` Payload Types for the
/// purpose of consistent encoding and decoding as well as identification within
/// a buffer.
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum DoipPayload<const N: usize> {
    /// `GenericNack` variant to hold `GenericNack` struct
    GenericNack(GenericNack),

    /// `VehicleIdentificationRequest` variant to hold `VehicleIdentificationRequest` struct
    VehicleIdentificationRequest(VehicleIdentificationRequest),

    /// `VehicleIdentificationRequestEid` variant to hold `VehicleIdentificationRequestEid` struct
    VehicleIdentificationRequestEid(VehicleIdentificationRequestEid),

    /// `VehicleIdentificationRequestVin` variant to hold `VehicleIdentificationRequestVin` struct
    VehicleIdentificationRequestVin(VehicleIdentificationRequestVin),

    /// `VehicleAnnouncementMessage` variant to hold `VehicleAnnouncementMessage` struct
    VehicleAnnouncementMessage(VehicleAnnouncementMessage),

    /// `RoutingActivationRequest` variant to hold `RoutingActivationRequest` struct
    RoutingActivationRequest(RoutingActivationRequest),

    /// `RoutingActivationResponse` variant to hold `RoutingActivationResponse` struct
    RoutingActivationResponse(RoutingActivationResponse),

    /// `AliveCheckRequest` variant to hold `AliveCheckRequest` struct
    AliveCheckRequest(AliveCheckRequest),

    /// `AliveCheckResponse` variant to hold `AliveCheckResponse` struct
    AliveCheckResponse(AliveCheckResponse),

    /// `EntityStatusRequest` variant to hold `EntityStatusRequest` struct
    EntityStatusRequest(EntityStatusRequest),

    /// `EntityStatusResponse` variant to hold `EntityStatusResponse` struct
    EntityStatusResponse(EntityStatusResponse),

    /// `PowerInformationRequest` variant to hold `PowerInformationRequest` struct
    PowerInformationRequest(PowerInformationRequest),

    /// `PowerInformationResponse` variant to hold `PowerInformationResponse` struct
    PowerInformationResponse(PowerInformationResponse),

    /// `DiagnosticMessage` variant to hold `DiagnosticMessage` struct
    DiagnosticMessage(DiagnosticMessage<N>),

    /// `DiagnosticMessageAck` variant to hold `DiagnosticMessageAck` struct
    DiagnosticMessageAck(DiagnosticMessageAck),

    /// `DiagnosticMessageNack` variant to hold `DiagnosticMessageNack` struct
    DiagnosticMessageNack(DiagnosticMessageNack),
}
