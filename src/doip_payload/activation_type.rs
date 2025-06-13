use crate::error::{Error, Result};

/// Used in Routing Activation Request to request specific routing types.
///
/// Used to customise the routing type requested from the `DoIP` entity for different
/// scenarios.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationType {
    /// Default
    Default = 0x00,

    /// WWH-OBD
    WwhObd = 0x01,

    /// Central Security
    CentralSecurity = 0x02,
}

impl TryFrom<&u8> for ActivationType {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == ActivationType::Default as u8 => Ok(ActivationType::Default),
            v if v == ActivationType::WwhObd as u8 => Ok(ActivationType::WwhObd),
            v if v == ActivationType::CentralSecurity as u8 => Ok(ActivationType::CentralSecurity),
            v => Err(Error::InvalidActivationType { value: v }),
        }
    }
}

impl From<ActivationType> for u8 {
    fn from(value: ActivationType) -> Self {
        value as u8
    }
}
