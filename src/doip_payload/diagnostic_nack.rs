use crate::error::{Error, Result};

/// Available negative acknowledgement codes for `DiagnosticMessageAck`.
///
/// Negative acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticNackCode {
    /// Reserved By ISO-13400 for bytes value `00`
    ReservedByIso13400_00 = 0x00,

    /// Reserved By ISO-13400 for bytes value `01`
    ReservedByIso13400_01 = 0x01,

    /// Invalid Source Address
    InvalidSourceAddress = 0x02,

    /// Unknown Target Address
    UnknownTargetAddress = 0x03,

    /// Diagnostic Message Too Large
    DiagnosticMessageTooLarge = 0x04,

    /// Out Of Memory
    OutOfMemory = 0x05,

    /// Target Unreachable
    TargetUnreachable = 0x06,

    /// Unknown Network
    UnknownNetwork = 0x07,

    /// Transport Protocol Error
    TransportProtocolError = 0x08,
}

impl From<DiagnosticNackCode> for u8 {
    fn from(value: DiagnosticNackCode) -> Self {
        value as u8
    }
}

impl TryFrom<&u8> for DiagnosticNackCode {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == DiagnosticNackCode::ReservedByIso13400_00 as u8 => {
                Ok(DiagnosticNackCode::ReservedByIso13400_00)
            }
            v if v == DiagnosticNackCode::ReservedByIso13400_01 as u8 => {
                Ok(DiagnosticNackCode::ReservedByIso13400_01)
            }
            v if v == DiagnosticNackCode::InvalidSourceAddress as u8 => {
                Ok(DiagnosticNackCode::InvalidSourceAddress)
            }
            v if v == DiagnosticNackCode::UnknownTargetAddress as u8 => {
                Ok(DiagnosticNackCode::UnknownTargetAddress)
            }
            v if v == DiagnosticNackCode::DiagnosticMessageTooLarge as u8 => {
                Ok(DiagnosticNackCode::DiagnosticMessageTooLarge)
            }
            v if v == DiagnosticNackCode::OutOfMemory as u8 => Ok(DiagnosticNackCode::OutOfMemory),
            v if v == DiagnosticNackCode::TargetUnreachable as u8 => {
                Ok(DiagnosticNackCode::TargetUnreachable)
            }
            v if v == DiagnosticNackCode::UnknownNetwork as u8 => {
                Ok(DiagnosticNackCode::UnknownNetwork)
            }
            v if v == DiagnosticNackCode::TransportProtocolError as u8 => {
                Ok(DiagnosticNackCode::TransportProtocolError)
            }
            v => Err(Error::InvalidDiagnosticNackCode { value: v }),
        }
    }
}
