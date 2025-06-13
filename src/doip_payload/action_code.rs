use crate::error::{Error, Result};

/// Used in Vehicle Announcement Messages to give next steps.
///
/// Used to inform the client of further actions which need to be taken on a
/// `DoIP` server.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActionCode {
    /// No Further Action Required
    NoFurtherActionRequired = 0x00,

    /// Reserved By ISO-13400 for bytes value `01`
    ReservedByIso13400_01 = 0x01,

    /// Reserved By ISO-13400 for bytes value `02`
    ReservedByIso13400_02 = 0x02,

    /// Reserved By ISO-13400 for bytes value `03`
    ReservedByIso13400_03 = 0x03,

    /// Reserved By ISO-13400 for bytes value `04`
    ReservedByIso13400_04 = 0x04,

    /// Reserved By ISO-13400 for bytes value `05`
    ReservedByIso13400_05 = 0x05,

    /// Reserved By ISO-13400 for bytes value `06`
    ReservedByIso13400_06 = 0x06,

    /// Reserved By ISO-13400 for bytes value `07`
    ReservedByIso13400_07 = 0x07,

    /// Reserved By ISO-13400 for bytes value `08`
    ReservedByIso13400_08 = 0x08,

    /// Reserved By ISO-13400 for bytes value `09`
    ReservedByIso13400_09 = 0x09,

    /// Reserved By ISO-13400 for bytes value `0A`
    ReservedByIso13400_0A = 0x0A,

    /// Reserved By ISO-13400 for bytes value `0B`
    ReservedByIso13400_0B = 0x0B,

    /// Reserved By ISO-13400 for bytes value `0C`
    ReservedByIso13400_0C = 0x0C,

    /// Reserved By ISO-13400 for bytes value `0D`
    ReservedByIso13400_0D = 0x0D,

    /// Reserved By ISO-13400 for bytes value `0E`
    ReservedByIso13400_0E = 0x0E,

    /// Reserved By ISO-13400 for bytes value `0F`
    ReservedByIso13400_0F = 0x0F,

    /// Routing Activation Required
    RoutingActivationRequired = 0x10,
}

impl TryFrom<&u8> for ActionCode {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == ActionCode::NoFurtherActionRequired as u8 => {
                Ok(ActionCode::NoFurtherActionRequired)
            }
            v if v == ActionCode::ReservedByIso13400_01 as u8 => {
                Ok(ActionCode::ReservedByIso13400_01)
            }
            v if v == ActionCode::ReservedByIso13400_02 as u8 => {
                Ok(ActionCode::ReservedByIso13400_02)
            }
            v if v == ActionCode::ReservedByIso13400_03 as u8 => {
                Ok(ActionCode::ReservedByIso13400_03)
            }
            v if v == ActionCode::ReservedByIso13400_04 as u8 => {
                Ok(ActionCode::ReservedByIso13400_04)
            }
            v if v == ActionCode::ReservedByIso13400_05 as u8 => {
                Ok(ActionCode::ReservedByIso13400_05)
            }
            v if v == ActionCode::ReservedByIso13400_06 as u8 => {
                Ok(ActionCode::ReservedByIso13400_06)
            }
            v if v == ActionCode::ReservedByIso13400_07 as u8 => {
                Ok(ActionCode::ReservedByIso13400_07)
            }
            v if v == ActionCode::ReservedByIso13400_08 as u8 => {
                Ok(ActionCode::ReservedByIso13400_08)
            }
            v if v == ActionCode::ReservedByIso13400_09 as u8 => {
                Ok(ActionCode::ReservedByIso13400_09)
            }
            v if v == ActionCode::ReservedByIso13400_0A as u8 => {
                Ok(ActionCode::ReservedByIso13400_0A)
            }
            v if v == ActionCode::ReservedByIso13400_0B as u8 => {
                Ok(ActionCode::ReservedByIso13400_0B)
            }
            v if v == ActionCode::ReservedByIso13400_0C as u8 => {
                Ok(ActionCode::ReservedByIso13400_0C)
            }
            v if v == ActionCode::ReservedByIso13400_0D as u8 => {
                Ok(ActionCode::ReservedByIso13400_0D)
            }
            v if v == ActionCode::ReservedByIso13400_0E as u8 => {
                Ok(ActionCode::ReservedByIso13400_0E)
            }
            v if v == ActionCode::ReservedByIso13400_0F as u8 => {
                Ok(ActionCode::ReservedByIso13400_0F)
            }
            v if v == ActionCode::RoutingActivationRequired as u8 => {
                Ok(ActionCode::RoutingActivationRequired)
            }
            v => Err(Error::InvalidActionCode { value: v }),
        }
    }
}

impl From<ActionCode> for u8 {
    fn from(value: ActionCode) -> Self {
        value as u8
    }
}
