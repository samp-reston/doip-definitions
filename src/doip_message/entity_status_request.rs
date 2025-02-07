use crate::{
    error::PayloadError,
    header::{DoipPayload, PayloadType},
};

/// Requests the status of a DoIP Entity.
#[derive(Copy, Clone, Debug)]
pub struct EntityStatusRequest {}

impl DoipPayload<'_> for EntityStatusRequest {
    fn payload_type(&self) -> PayloadType {
        PayloadType::EntityStatusRequest
    }

    fn to_bytes(&self, buffer: &mut [u8]) -> Result<usize, PayloadError> {
        let _ = buffer;
        Ok(0)
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Self, PayloadError> {
        Ok(Self {})
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        doip_message::entity_status_request::EntityStatusRequest,
        header::{DoipPayload, PayloadType},
    };

    #[test]
    fn test_payload_type() {
        let request = EntityStatusRequest {};
        assert_eq!(request.payload_type(), PayloadType::EntityStatusRequest);
    }

    #[test]
    fn test_to_bytes() {
        let mut buffer = [0; 1024];
        let request = EntityStatusRequest {};
        assert_eq!(request.to_bytes(&mut buffer), Ok(0));
    }

    #[test]
    fn test_from_bytes_ok() {
        let mut buffer = [0; 1024];
        let bytes = EntityStatusRequest {}.to_bytes(&mut buffer).unwrap();
        let request = EntityStatusRequest::from_bytes(&buffer[..bytes]);

        assert!(
            request.is_ok(),
            "Expected EntityStatusRequest, recieved an Error."
        );
    }
}
