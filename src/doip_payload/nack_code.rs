use crate::error::{Error, Result};

/// Used in `GenericNack`, `NackCode` provides the possible errors causing the
/// NACK.
///
/// Used to understand the result of a `DoIP` packet.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NackCode {
    /// Incorrect Pattern Format
    IncorrectPatternFormat = 0x00,

    /// Unknown Payload Type
    UnknownPayloadType = 0x01,

    /// Message Too Large
    MessageTooLarge = 0x02,

    /// Out Of Memory
    OutOfMemory = 0x03,

    /// Invalid Payload Length
    InvalidPayloadLength = 0x04,
}

impl From<NackCode> for u8 {
    fn from(value: NackCode) -> Self {
        value as u8
    }
}

impl TryFrom<&u8> for NackCode {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == NackCode::IncorrectPatternFormat as u8 => {
                Ok(NackCode::IncorrectPatternFormat)
            }
            v if v == NackCode::UnknownPayloadType as u8 => Ok(NackCode::UnknownPayloadType),
            v if v == NackCode::MessageTooLarge as u8 => Ok(NackCode::MessageTooLarge),
            v if v == NackCode::OutOfMemory as u8 => Ok(NackCode::OutOfMemory),
            v if v == NackCode::InvalidPayloadLength as u8 => Ok(NackCode::InvalidPayloadLength),
            v => Err(Error::InvalidNackCode { value: v }),
        }
    }
}
