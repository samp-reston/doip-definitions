use crate::{
    definitions::{
        DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN, DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN,
        DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN,
    },
    error::{Error, Result},
    payload::NodeType,
};

/// Expected reponse from `EntityStatusRequest`.
///
/// Containing details of the target of the `EntityStatusRequest`, the
/// `EntityStatusResponse` provides the program with details pertaining to the
/// active status of the entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityStatusResponse {
    /// The type of entity, either a `Gateway` or `Node`
    pub node_type: NodeType,

    /// The number of maximum concurrent TCP sockets allowed on this entity
    pub max_concurrent_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN],

    /// The number of currently open TCP sockets on the entity
    pub currently_open_sockets: [u8; DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN],

    /// The max data size allowed to be sent to the entity
    pub max_data_size: [u8; DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN],
}

impl From<EntityStatusResponse>
    for [u8; 1
        + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
        + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
        + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN]
{
    fn from(value: EntityStatusResponse) -> Self {
        let mut buffer = [0u8; 1
            + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN
            + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN];
        let mut offset = 0;

        buffer[offset] = value.node_type.into();
        offset += 1;

        #[allow(clippy::range_plus_one)]
        buffer[offset..offset + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN]
            .copy_from_slice(&value.max_concurrent_sockets);
        offset += DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN;

        #[allow(clippy::range_plus_one)]
        buffer[offset..offset + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN]
            .copy_from_slice(&value.currently_open_sockets);
        offset += DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN;

        buffer[offset..offset + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN]
            .copy_from_slice(&value.max_data_size);

        buffer
    }
}

impl TryFrom<&[u8]> for EntityStatusResponse {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let node_type = value
            .get(offset)
            .ok_or(Error::OutOfBounds {
                source: "EntityStatusResponse",
                variable: "Node Type",
            })?
            .try_into()?;

        let max_concurrent_sockets = value
            .get(offset..DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN)
            .ok_or(Error::OutOfBounds {
                source: "EntityStatusResponse",
                variable: "Max Concurrent Sockets",
            })?
            .try_into()?;

        offset += DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN;

        #[allow(clippy::range_plus_one)]
        let currently_open_sockets = value
            .get(offset..offset + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN)
            .ok_or(Error::OutOfBounds {
                source: "EntityStatusResponse",
                variable: "Currently Open Sockets",
            })?
            .try_into()?;

        offset += DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN;

        let max_data_size = value
            .get(offset..offset + DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN)
            .ok_or(Error::OutOfBounds {
                source: "EntityStatusResponse",
                variable: "Max Data Size",
            })?
            .try_into()?;

        Ok(EntityStatusResponse {
            node_type,
            max_concurrent_sockets,
            currently_open_sockets,
            max_data_size,
        })
    }
}
