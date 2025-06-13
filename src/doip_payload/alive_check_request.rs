/// Checks the TCP Socket is still alive
///
/// Sent with no payload, the `AliveCheckRequest` is utilised to maintain a connection
/// to a TCP socket or to check the status of one.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AliveCheckRequest {}

impl From<AliveCheckRequest> for [u8; 0] {
    fn from(_: AliveCheckRequest) -> Self {
        []
    }
}

impl From<&[u8]> for AliveCheckRequest {
    fn from(_: &[u8]) -> Self {
        AliveCheckRequest {}
    }
}
