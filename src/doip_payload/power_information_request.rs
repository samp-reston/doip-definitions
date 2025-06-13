/// Requests the power mode status of a `DoIP` Entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PowerInformationRequest {}

impl From<PowerInformationRequest> for [u8; 0] {
    fn from(_: PowerInformationRequest) -> Self {
        []
    }
}

impl From<&[u8]> for PowerInformationRequest {
    fn from(_: &[u8]) -> Self {
        PowerInformationRequest {}
    }
}
