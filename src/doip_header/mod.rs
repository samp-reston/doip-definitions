pub mod payload_type;
pub mod version;

use payload_type::PayloadType;
use version::ProtocolVersion;

use crate::error::{Error, Result};

/// The definitive fields of a `DoIP` frame.
///
/// The definition of a `DoIP` frame is found in the `DoipHeader`, this contains each
/// key field which a parser uses to identify the bytes which pertain to a `DoIP`
/// frame.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
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

impl TryFrom<[u8; 8]> for DoipHeader {
    type Error = Error;

    fn try_from(value: [u8; 8]) -> Result<Self> {
        let protocol_version_slice = value.first().ok_or(Error::OutOfBounds {
            source: "DoIP Header",
            variable: "Protocol Version",
        })?;
        let protocol_version = ProtocolVersion::try_from(protocol_version_slice)?;

        let inverse_protocol_version_slice = value.get(1).ok_or(Error::OutOfBounds {
            source: "DoIP Header",
            variable: "Inverse Protocol Version",
        })?;
        let inverse_protocol_version = *inverse_protocol_version_slice;

        let payload_type_slice = value.get(2..4).ok_or(Error::OutOfBounds {
            source: "DoIP Header",
            variable: "Payload Type",
        })?;
        let payload_type = PayloadType::try_from(payload_type_slice)?;

        let payload_length_slice: [u8; 4] = value
            .get(4..8)
            .ok_or(Error::OutOfBounds {
                source: "DoIP Header",
                variable: "Payload Length",
            })?
            .try_into()?;
        let payload_length = u32::from_be_bytes(payload_length_slice);

        Ok(DoipHeader {
            protocol_version,
            inverse_protocol_version,
            payload_type,
            payload_length,
        })
    }
}

impl From<DoipHeader> for [u8; 8] {
    fn from(value: DoipHeader) -> Self {
        let protocol_version: u8 = u8::from(value.protocol_version);
        let inverse_protocol_version: u8 = value.inverse_protocol_version;
        let payload_type: [u8; 2] = <[u8; 2]>::from(value.payload_type);
        let payload_length: [u8; 4] = value.payload_length.to_be_bytes();

        [
            protocol_version,
            inverse_protocol_version,
            payload_type[0],
            payload_type[1],
            payload_length[0],
            payload_length[1],
            payload_length[2],
            payload_length[3],
        ]
    }
}
