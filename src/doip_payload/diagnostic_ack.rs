use crate::error::{Error, Result};

/// Available positive acknowledgement codes for `DiagnosticMessageAck`.
///
/// Positive acknowledgement codes from the result of a sent `DiagnosticMessage`.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticAckCode {
    /// Acknowledged
    Acknowledged = 0x00,
}

impl From<DiagnosticAckCode> for u8 {
    fn from(value: DiagnosticAckCode) -> Self {
        value as u8
    }
}

impl TryFrom<&u8> for DiagnosticAckCode {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == DiagnosticAckCode::Acknowledged as u8 => Ok(DiagnosticAckCode::Acknowledged),
            v => Err(Error::InvalidDiagnosticAckCode { value: v }),
        }
    }
}
