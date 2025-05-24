use crate::{
    definitions::{DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN, DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN},
    payload::ActivationType,
};

/// Request for routing activation.
///
/// Usually routing activation is required for `DoipMessage` passing to a `DoIP` server,
/// the `RoutingActivationRequest` details the activation type required.
#[cfg_attr(feature = "std", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RoutingActivationRequest {
    /// Source address of the requesting entity
    pub source_address: [u8; DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN],

    /// Activation type
    pub activation_type: ActivationType,

    /// ISO reserved buffer, currently left empty
    pub buffer: [u8; DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN],
}
