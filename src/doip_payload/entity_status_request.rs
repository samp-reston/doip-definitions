/// Requests the status of a `DoIP` Entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityStatusRequest {}

impl From<EntityStatusRequest> for [u8; 0] {
    fn from(_: EntityStatusRequest) -> Self {
        []
    }
}

impl From<&[u8]> for EntityStatusRequest {
    fn from(_: &[u8]) -> Self {
        EntityStatusRequest {}
    }
}
