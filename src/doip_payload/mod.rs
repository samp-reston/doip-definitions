// region:      --- Modules
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

// -- Flatten

pub use alive_check_request::*;
pub use alive_check_response::*;
pub use diagnostic_message::*;
pub use diagnostic_message_ack::*;
pub use diagnostic_message_nack::*;
pub use entity_status_request::*;
pub use entity_status_response::*;
pub use generic_nack::*;
pub use power_information_request::*;
pub use power_information_response::*;
pub use routing_activation_request::*;
pub use routing_activation_response::*;
pub use vehicle_announcement_message::*;
pub use vehicle_identification_request::*;
pub use vehicle_identification_request_eid::*;
pub use vehicle_identification_request_vin::*;

// endregion:      --- Modules

/// Implemented across `DoIP` Payload Types for consistent encoding and decoding of buffers.
///
/// `DoipPayload` is implemented for all the `DoIP` Payload Types for the
/// purpose of consistent encoding and decoding as well as identification within
/// a buffer.
#[cfg(not(feature = "std"))]
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

/// Implemented across `DoIP` Payload Types for consistent encoding and decoding of buffers.
///
/// `DoipPayload` is implemented for all the `DoIP` Payload Types for the
/// purpose of consistent encoding and decoding as well as identification within
/// a buffer.
#[cfg(feature = "std")]
#[derive(Debug, PartialEq, Clone)]
pub enum DoipPayload {
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
    DiagnosticMessage(DiagnosticMessage),

    /// `DiagnosticMessageAck` variant to hold `DiagnosticMessageAck` struct
    DiagnosticMessageAck(DiagnosticMessageAck),

    /// `DiagnosticMessageNack` variant to hold `DiagnosticMessageNack` struct
    DiagnosticMessageNack(DiagnosticMessageNack),
}
