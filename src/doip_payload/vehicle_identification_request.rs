/// Used to request a `VehicleAnnouncement` from all available `DoIP` entities
/// on the network.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VehicleIdentificationRequest {}

impl From<VehicleIdentificationRequest> for [u8; 0] {
    fn from(_: VehicleIdentificationRequest) -> Self {
        []
    }
}

impl From<&[u8]> for VehicleIdentificationRequest {
    fn from(_: &[u8]) -> Self {
        VehicleIdentificationRequest {}
    }
}
