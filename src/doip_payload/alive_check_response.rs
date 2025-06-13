use crate::definitions::DOIP_DIAG_COMMON_SOURCE_LEN;
use crate::error::{Error, Result};

/// Confirmation of the `AliveCheckRequest`.
///
/// The typical response from an `AliveCheckRequest`.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AliveCheckResponse {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],
}

impl From<AliveCheckResponse> for [u8; DOIP_DIAG_COMMON_SOURCE_LEN] {
    fn from(value: AliveCheckResponse) -> Self {
        value.source_address
    }
}

impl TryFrom<&[u8]> for AliveCheckResponse {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN] = value
            .get(0..DOIP_DIAG_COMMON_SOURCE_LEN)
            .ok_or(Error::OutOfBounds {
                source: "AliveCheckResponse",
                variable: "Source Address",
            })?
            .try_into()?;

        Ok(AliveCheckResponse { source_address })
    }
}
