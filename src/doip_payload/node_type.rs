use crate::error::{Error, Result};

/// Used in `EntityStatusResponse`, `NodeType` provides the possibilities of the
/// `node_type` field.
///
/// Used to understand the result of a `DoIP` packet.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
// Node Type
pub enum NodeType {
    /// Doip Gateway
    DoipGateway = 0x00,

    /// Doip Node
    DoipNode = 0x01,
}

impl TryFrom<&u8> for NodeType {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == NodeType::DoipGateway as u8 => Ok(NodeType::DoipGateway),
            v if v == NodeType::DoipNode as u8 => Ok(NodeType::DoipNode),
            v => Err(Error::InvalidNodeType { value: v }),
        }
    }
}

impl From<NodeType> for u8 {
    fn from(value: NodeType) -> Self {
        value as u8
    }
}
