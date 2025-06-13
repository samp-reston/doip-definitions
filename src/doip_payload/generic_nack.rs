use crate::{
    error::{Error, Result},
    payload::NackCode,
};

/// The generic negative acknowledgement of a bad request.
///
/// This is found usually when a critical error occurs due to a bad `DoIP` packet
/// or an entity issue.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericNack {
    /// Available negative acknowledgement codes
    pub nack_code: NackCode,
}

impl From<GenericNack> for [u8; 1] {
    fn from(value: GenericNack) -> Self {
        let nack_code: u8 = value.nack_code.into();
        [nack_code]
    }
}

impl TryFrom<&[u8]> for GenericNack {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let nack_code_slice = value.first().ok_or(Error::OutOfBounds {
            source: "GenericNack",
            variable: "Nack Code",
        })?;

        let nack_code = NackCode::try_from(nack_code_slice)?;

        Ok(GenericNack { nack_code })
    }
}
