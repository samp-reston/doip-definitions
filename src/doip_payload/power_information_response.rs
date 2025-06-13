use crate::{
    error::{Error, Result},
    payload::PowerMode,
};

/// Expected reponse from `PowerInformationRequest`.
///
/// Containing details of the target of the `PowerInformationRequest`, the
/// `EntityStatusResponse` provides the program with details pertaining to the
/// active power mode status of the entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PowerInformationResponse {
    /// Possible power modes available
    pub power_mode: PowerMode,
}

impl From<PowerInformationResponse> for [u8; 1] {
    fn from(value: PowerInformationResponse) -> Self {
        let power_mode: u8 = value.power_mode.into();
        [power_mode]
    }
}

impl TryFrom<&[u8]> for PowerInformationResponse {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let power_mode_slice = value.first().ok_or(Error::OutOfBounds {
            source: "PowerInformationResponse",
            variable: "Power Mode",
        })?;

        let power_mode = PowerMode::try_from(power_mode_slice)?;

        Ok(PowerInformationResponse { power_mode })
    }
}
