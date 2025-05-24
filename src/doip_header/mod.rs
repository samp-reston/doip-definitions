use automotive_diag::doip::{PayloadType, ProtocolVersion};

/// The definitive fields of a `DoIP` frame.
///
/// The definition of a `DoIP` frame is found in the `DoipHeader`, this contains each
/// key field which a parser uses to identify the bytes which pertain to a `DoIP`
/// frame.
#[cfg_attr(feature = "std", pyo3::pyclass)]
#[derive(Debug, PartialEq, Clone)]
pub struct DoipHeader {
    /// `protocol_version` acts a pair with the `inverse_protocol_version` to create
    /// a validation check to ensure the packet is constructed correctly. There
    /// are specific versions available within the `DoipVersion` enum.
    pub protocol_version: ProtocolVersion,

    /// Calculated using bitwise inversion, the `inverse_protocol_version` acts
    /// as a validation technique for validating the packet.
    pub inverse_protocol_version: u8,

    /// The type of payload alongside the header, this defines what is contained
    /// within the message, and directs the parser to accurately identify fields.
    ///
    /// A list of valid Payload Types are available in the `PayloadType` enum.
    pub payload_type: PayloadType,

    /// The `payload_length` is automatically calulated on the construction of a
    /// `DoipHeader` and is taken from the payload the header was initiated with.
    ///
    /// This tells the parser how many bytes to expect after the header.
    pub payload_length: u32,
}
