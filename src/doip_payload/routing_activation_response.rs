use crate::{
    definitions::{
        DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN, DOIP_ROUTING_ACTIVATION_RES_ISO_LEN,
        DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN,
    },
    payload::ActivationCode,
};

/// A response to the `RoutingActivationRequest`.
///
/// Contains the logical address of the recieved `DoIP` entity along with the activation code.
#[cfg_attr(feature = "std", pyo3::pyclass)]
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
