use crate::definitions::DOIP_COMMON_EID_LEN;
use crate::error::{Error, Result};

/// Requests a `VehicleAnnouncementMessage` from entities with the same EID
///
/// Matches `DoIP` entities with the same EID for response to the request.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VehicleIdentificationRequestEid {
    /// Entity Identification
    pub eid: [u8; DOIP_COMMON_EID_LEN],
}

impl From<VehicleIdentificationRequestEid> for [u8; DOIP_COMMON_EID_LEN] {
    fn from(value: VehicleIdentificationRequestEid) -> Self {
        value.eid
    }
}

impl TryFrom<&[u8]> for VehicleIdentificationRequestEid {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let eid: [u8; DOIP_COMMON_EID_LEN] = value
            .get(0..DOIP_COMMON_EID_LEN)
            .ok_or(Error::OutOfBounds {
                source: "VehicleIdentificationRequestEid",
                variable: "Eid",
            })?
            .try_into()?;

        Ok(VehicleIdentificationRequestEid { eid })
    }
}
