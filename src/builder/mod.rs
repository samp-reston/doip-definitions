use crate::header::{PayloadType, ProtocolVersion};
use crate::payload::AliveCheckRequest;
use crate::{header::DoipHeader, message::DoipMessage, payload::DoipPayload};

/// A builder for constructing `DoipMessage` instances with specified headers and payloads.
///
/// This struct provides a fluent interface to configure the protocol version,
/// payload, and automatically populate corresponding header fields based on the payload.
#[derive(Default, Debug)]
pub struct DoipMessageBuilder {
    /// The header portion of the `DoIP` message, containing metadata like protocol version and payload type.
    header: DoipHeader,

    /// The payload content of the `DoIP` message, which varies depending on the message type.
    payload: DoipPayload,
}

impl Default for DoipPayload {
    /// Provides a default payload of type `AliveCheckRequest`.
    fn default() -> Self {
        DoipPayload::AliveCheckRequest(AliveCheckRequest {})
    }
}

impl Default for DoipHeader {
    /// Constructs a default `DoipHeader` with:
    /// - Protocol version set to `DefaultValue`
    /// - Inverse protocol version calculated automatically
    /// - Payload type set to `AliveCheckRequest`
    /// - Payload length initialized to 0
    fn default() -> Self {
        Self {
            protocol_version: ProtocolVersion::DefaultValue,
            inverse_protocol_version: !(ProtocolVersion::DefaultValue as u8),
            payload_type: PayloadType::AliveCheckRequest,
            payload_length: Default::default(),
        }
    }
}

impl DoipMessageBuilder {
    /// Creates a new `DoipMessageBuilder` instance using the default header and payload.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// let builder = DoipMessageBuilder::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        DoipMessageBuilder::default()
    }

    /// Sets the protocol version in the header and updates the inverse protocol version accordingly.
    ///
    /// # Arguments
    ///
    /// * `protocol_version` - An object that can be converted into a `ProtocolVersion`.
    ///
    /// # Returns
    ///
    /// The updated builder instance for chaining.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// use doip_definitions::header::ProtocolVersion;
    /// let builder = DoipMessageBuilder::new().protocol_version(ProtocolVersion::Iso13400_2012);
    /// ```
    #[must_use]
    pub fn protocol_version(mut self, protocol_version: impl Into<ProtocolVersion>) -> Self {
        self.header.protocol_version = protocol_version.into();
        self.header.inverse_protocol_version = !(self.header.protocol_version as u8);
        self
    }

    /// Sets the payload of the message and updates the header's payload type and length accordingly.
    ///
    /// # Arguments
    ///
    /// * `payload` - An object that can be converted into a `DoipPayload`.
    ///
    /// # Returns
    ///
    /// The updated builder instance for chaining.
    ///
    /// # Panics
    ///
    /// This method panics if the size of the provided payload cannot be determined using
    /// [`core::mem::size_of_val`]—which may occur if dynamically sized types or trait objects
    /// are passed in as payload components.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// use doip_definitions::header::PayloadType;
    /// use doip_definitions::payload::AliveCheckRequest;
    /// use doip_definitions::payload::DoipPayload;
    ///
    /// let builder = DoipMessageBuilder::new().payload(DoipPayload::AliveCheckRequest(AliveCheckRequest {}));
    /// ```
    #[must_use]
    pub fn payload(mut self, payload: impl Into<DoipPayload>) -> Self {
        self.payload = payload.into();

        let (payload_type, size) = match self.payload {
            DoipPayload::GenericNack(ref pay) => (PayloadType::GenericNack, size_of_val(pay)),
            DoipPayload::VehicleIdentificationRequest(ref pay) => {
                (PayloadType::VehicleIdentificationRequest, size_of_val(pay))
            }
            DoipPayload::VehicleIdentificationRequestEid(ref pay) => (
                PayloadType::VehicleIdentificationRequestEid,
                size_of_val(pay),
            ),
            DoipPayload::VehicleIdentificationRequestVin(ref pay) => (
                PayloadType::VehicleIdentificationRequestVin,
                size_of_val(pay),
            ),
            DoipPayload::VehicleAnnouncementMessage(ref pay) => {
                (PayloadType::VehicleAnnouncementMessage, size_of_val(pay))
            }
            DoipPayload::RoutingActivationRequest(ref pay) => {
                (PayloadType::RoutingActivationRequest, size_of_val(pay))
            }
            DoipPayload::RoutingActivationResponse(ref pay) => {
                (PayloadType::RoutingActivationResponse, size_of_val(pay))
            }
            DoipPayload::AliveCheckRequest(ref pay) => {
                (PayloadType::AliveCheckRequest, size_of_val(pay))
            }
            DoipPayload::AliveCheckResponse(ref pay) => {
                (PayloadType::AliveCheckResponse, size_of_val(pay))
            }
            DoipPayload::EntityStatusRequest(ref pay) => {
                (PayloadType::EntityStatusRequest, size_of_val(pay))
            }
            DoipPayload::EntityStatusResponse(ref pay) => {
                (PayloadType::EntityStatusResponse, size_of_val(pay))
            }
            DoipPayload::PowerInformationRequest(ref pay) => {
                (PayloadType::PowerInformationRequest, size_of_val(pay))
            }
            DoipPayload::PowerInformationResponse(ref pay) => {
                (PayloadType::PowerInformationResponse, size_of_val(pay))
            }
            DoipPayload::DiagnosticMessage(ref pay) => {
                (PayloadType::DiagnosticMessage, size_of_val(pay))
            }
            DoipPayload::DiagnosticMessageAck(ref pay) => {
                (PayloadType::DiagnosticMessageAck, size_of_val(pay))
            }
            DoipPayload::DiagnosticMessageNack(ref pay) => {
                (PayloadType::DiagnosticMessageNack, size_of_val(pay))
            }
        };

        self.header.payload_type = payload_type;
        self.header.payload_length =
            u32::try_from(size).expect("This should never be larger than u32.");

        self
    }

    /// Finalizes the builder and returns the constructed `DoipMessage`.
    ///
    /// # Returns
    ///
    /// The fully constructed `DoipMessage`.
    ///
    /// # Example
    /// ```
    /// use doip_definitions::builder::DoipMessageBuilder;
    /// use doip_definitions::header::PayloadType;
    /// use doip_definitions::payload::AliveCheckRequest;
    /// use doip_definitions::payload::DoipPayload;
    ///
    /// let message = DoipMessageBuilder::new()
    ///     .payload(DoipPayload::AliveCheckRequest(AliveCheckRequest {}))
    ///     .build();
    /// ```
    #[must_use]
    pub fn build(self) -> DoipMessage {
        DoipMessage {
            header: self.header,
            payload: self.payload,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
        payload::{DiagnosticMessage, DoipPayload},
    };

    #[test]
    fn test_diagnostic_message_payload() {
        let message = vec![0x01, 0x02, 0x03];
        let message_length = message.len();
        let payload = DoipPayload::DiagnosticMessage(DiagnosticMessage {
            source_address: [0x12, 0x34],
            target_address: [0x56, 0x78],
            message,
        });
        let builder = DoipMessageBuilder::new().payload(payload);
        let doip_message = builder.build();

        assert_eq!(
            doip_message.header.payload_type,
            PayloadType::DiagnosticMessage
        );
        assert_eq!(
            doip_message.header.payload_length,
            (DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + message_length) as u32
        );
    }
}
