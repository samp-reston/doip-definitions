use crate::{
    definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN},
    error::{Error, Result},
    payload::DiagnosticNackCode,
};

/// Negative acknowledgement of a `DiagnosticMessage`.
///
/// Containing the source and target entity addresses, as well as the `DiagnosticNackCode`
/// for the `DiagnosticMessage` initially sent by the target entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DiagnosticMessageNack {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// The negative acknowledgement code
    pub nack_code: DiagnosticNackCode,
}

impl From<DiagnosticMessageNack>
    for [u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1]
{
    fn from(value: DiagnosticMessageNack) -> Self {
        let mut buffer = [0u8; DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + 1];

        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&value.source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN].copy_from_slice(&value.target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset] = value.nack_code.into();

        buffer
    }
}

impl TryFrom<&[u8]> for DiagnosticMessageNack {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let source_address = value
            .get(offset..DOIP_DIAG_COMMON_SOURCE_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessageNack",
                variable: "Source Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        let target_address = value
            .get(offset..offset + DOIP_DIAG_COMMON_TARGET_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessageNack",
                variable: "Target Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        let nack_code = value
            .get(offset)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessageNack",
                variable: "Nack Code",
            })?
            .try_into()?;

        Ok(DiagnosticMessageNack {
            source_address,
            target_address,
            nack_code,
        })
    }
}
