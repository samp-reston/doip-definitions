use crate::error::{Error, Result};

/// Used in `PowerInformationResponse`, `PowerMode` provides the power mode that
/// the `DoIP` entity can be.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PowerMode {
    /// Not Ready
    NotReady = 0x00,

    /// Ready
    Ready = 0x01,

    /// Not Supported
    NotSupported = 0x02,
}

impl From<PowerMode> for u8 {
    fn from(value: PowerMode) -> Self {
        value as u8
    }
}

impl TryFrom<&u8> for PowerMode {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == PowerMode::NotReady as u8 => Ok(PowerMode::NotReady),
            v if v == PowerMode::Ready as u8 => Ok(PowerMode::Ready),
            v if v == PowerMode::NotSupported as u8 => Ok(PowerMode::NotSupported),
            v => Err(Error::InvalidPowerMode { value: v }),
        }
    }
}
