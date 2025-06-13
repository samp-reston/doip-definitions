use crate::{
    definitions::{DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN},
    error::{Error, Result},
    payload::ActivationType,
};

/// Request for routing activation.
///
/// Usually routing activation is required for `DoipMessage` passing to a `DoIP` server,
/// the `RoutingActivationRequest` details the activation type required.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RoutingActivationRequest {
    /// Source address of the requesting entity
    pub source_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN],

    /// Activation type
    pub activation_type: ActivationType,

    /// ISO reserved buffer, currently left empty
    pub buffer: [u8; DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN],
}

impl From<RoutingActivationRequest>
    for [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN + 1 + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN]
{
    fn from(value: RoutingActivationRequest) -> Self {
        let mut buffer =
            [0u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN + 1 + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN];

        let mut offset = 0;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN]
            .copy_from_slice(&value.source_address);
        offset += DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;

        buffer[offset] = value.activation_type.into();
        offset += 1;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN].copy_from_slice(&value.buffer);

        buffer
    }
}

impl TryFrom<&[u8]> for RoutingActivationRequest {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let source_address = value
            .get(offset..DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN)
            .ok_or(Error::OutOfBounds {
                source: "RoutingActivationRequest",
                variable: "Source Address",
            })?
            .try_into()?;

        offset += DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;

        let activation_type = value
            .get(offset)
            .ok_or(Error::OutOfBounds {
                source: "RoutingActivationRequest",
                variable: "Vin",
            })?
            .try_into()?;

        offset += 1;

        let buffer = value
            .get(offset..offset + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN)
            .ok_or(Error::OutOfBounds {
                source: "RoutingActivationResponse",
                variable: "Buffer",
            })?
            .try_into()?;

        Ok(RoutingActivationRequest {
            source_address,
            activation_type,
            buffer,
        })
    }
}
