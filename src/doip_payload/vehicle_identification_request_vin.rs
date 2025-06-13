use crate::definitions::DOIP_COMMON_VIN_LEN;
use crate::error::{Error, Result};

/// Requests a `VehicleAnnouncementMessage` from entities with the same VIN
///
/// Matches `DoIP` entities with the same VIN for response to the request.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VehicleIdentificationRequestVin {
    /// Vehicle Identification Number
    pub vin: [u8; DOIP_COMMON_VIN_LEN],
}

impl From<VehicleIdentificationRequestVin> for [u8; DOIP_COMMON_VIN_LEN] {
    fn from(value: VehicleIdentificationRequestVin) -> Self {
        value.vin
    }
}

impl TryFrom<&[u8]> for VehicleIdentificationRequestVin {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let vin: [u8; DOIP_COMMON_VIN_LEN] = value
            .get(0..DOIP_COMMON_VIN_LEN)
            .ok_or(Error::OutOfBounds {
                source: "VehicleIdentificationRequestVin",
                variable: "Vin",
            })?
            .try_into()?;

        Ok(VehicleIdentificationRequestVin { vin })
    }
}
