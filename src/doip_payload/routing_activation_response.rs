use crate::{
    definitions::{
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN, DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN,
    },
    error::{Error, Result},
    payload::ActivationCode,
};

/// A response to the `RoutingActivationRequest`.
///
/// Contains the logical address of the recieved `DoIP` entity along with the activation code.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RoutingActivationResponse {
    /// Logical address of requested entity
    pub logical_address: [u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN],

    /// Source address of response entity
    pub source_address: [u8; DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN],

    /// Activation Code
    pub activation_code: ActivationCode,

    /// ISO reserved buffer
    pub buffer: [u8; DOIP_ROUTING_ACTIVATION_RES_ISO_LEN],
}

impl From<RoutingActivationResponse>
    for [u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN
        + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN
        + 1
        + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN]
{
    fn from(value: RoutingActivationResponse) -> Self {
        let mut buffer = [0u8; DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN
            + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN
            + 1
            + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN];

        let mut offset = 0;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN]
            .copy_from_slice(&value.logical_address);
        offset += DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN]
            .copy_from_slice(&value.source_address);
        offset += DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN;

        buffer[offset] = value.activation_code.into();
        offset += 1;

        buffer[offset..offset + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN].copy_from_slice(&value.buffer);

        buffer
    }
}

impl TryFrom<&[u8]> for RoutingActivationResponse {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let logical_address = value
            .get(offset..DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN)
            .ok_or(Error::OutOfBounds {
                source: "RoutingActivationResponse",
                variable: "Logical Address",
            })?
            .try_into()?;

        offset += DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN;

        let source_address = value
            .get(offset..offset + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN)
            .ok_or(Error::OutOfBounds {
                source: "RoutingActivationResponse",
                variable: "Source Address",
            })?
            .try_into()?;

        offset += DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN;

        let activation_code = value
            .get(offset)
            .ok_or(Error::OutOfBounds {
                source: "RoutingActivationResponse",
                variable: "Activation Code",
            })?
            .try_into()?;

        offset += 1;

        let buffer = value
            .get(offset..offset + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN)
            .ok_or(Error::OutOfBounds {
                source: "RoutingActivationResponse",
                variable: "Buffer",
            })?
            .try_into()?;

        Ok(RoutingActivationResponse {
            logical_address,
            source_address,
            activation_code,
            buffer,
        })
    }
}
