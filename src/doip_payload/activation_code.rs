use crate::error::{Error, Result};

/// Used in Routing Activation Response for results from a Routing Activation
/// Request.
///
/// Used to understand the result of a Routing Activation Request to understand
/// which logical route to take.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationCode {
    /// Denied Unknown Source Address
    DeniedUnknownSourceAddress = 0x00,

    /// Denied TCP Sockets Full
    DeniedTCPSocketsFull = 0x01,

    /// Denied TCP Socket Already Connected
    DeniedTCPSocketAlreadyConnected = 0x02,

    /// Denied Source Is Already Active
    DeniedSourceIsAlreadyActive = 0x03,

    /// Denied Missing Authentication
    DeniedMissingAuthentication = 0x04,

    /// Denied Rejected Confirmation
    DeniedRejectedConfirmation = 0x05,

    /// Denied Unsupported Routing `ActivationType`
    DeniedUnsupportedRoutingActivationType = 0x06,

    /// Denied Request Encrypted TLS Connection
    DeniedRequestEncryptedTLSConnection = 0x07,

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

    /// Successfully Activated
    SuccessfullyActivated = 0x10,

    /// Activated Confirmation Required
    ActivatedConfirmationRequired = 0x11,
}

impl TryFrom<&u8> for ActivationCode {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == ActivationCode::DeniedUnknownSourceAddress as u8 => {
                Ok(ActivationCode::DeniedUnknownSourceAddress)
            }
            v if v == ActivationCode::DeniedTCPSocketsFull as u8 => {
                Ok(ActivationCode::DeniedTCPSocketsFull)
            }
            v if v == ActivationCode::DeniedTCPSocketAlreadyConnected as u8 => {
                Ok(ActivationCode::DeniedTCPSocketAlreadyConnected)
            }
            v if v == ActivationCode::DeniedSourceIsAlreadyActive as u8 => {
                Ok(ActivationCode::DeniedSourceIsAlreadyActive)
            }
            v if v == ActivationCode::DeniedMissingAuthentication as u8 => {
                Ok(ActivationCode::DeniedMissingAuthentication)
            }
            v if v == ActivationCode::DeniedRejectedConfirmation as u8 => {
                Ok(ActivationCode::DeniedRejectedConfirmation)
            }
            v if v == ActivationCode::DeniedUnsupportedRoutingActivationType as u8 => {
                Ok(ActivationCode::DeniedUnsupportedRoutingActivationType)
            }
            v if v == ActivationCode::DeniedRequestEncryptedTLSConnection as u8 => {
                Ok(ActivationCode::DeniedRequestEncryptedTLSConnection)
            }
            v if v == ActivationCode::ReservedByIso13400_08 as u8 => {
                Ok(ActivationCode::ReservedByIso13400_08)
            }
            v if v == ActivationCode::ReservedByIso13400_09 as u8 => {
                Ok(ActivationCode::ReservedByIso13400_09)
            }
            v if v == ActivationCode::ReservedByIso13400_0A as u8 => {
                Ok(ActivationCode::ReservedByIso13400_0A)
            }
            v if v == ActivationCode::ReservedByIso13400_0B as u8 => {
                Ok(ActivationCode::ReservedByIso13400_0B)
            }
            v if v == ActivationCode::ReservedByIso13400_0C as u8 => {
                Ok(ActivationCode::ReservedByIso13400_0C)
            }
            v if v == ActivationCode::ReservedByIso13400_0D as u8 => {
                Ok(ActivationCode::ReservedByIso13400_0D)
            }
            v if v == ActivationCode::ReservedByIso13400_0E as u8 => {
                Ok(ActivationCode::ReservedByIso13400_0E)
            }
            v if v == ActivationCode::ReservedByIso13400_0F as u8 => {
                Ok(ActivationCode::ReservedByIso13400_0F)
            }
            v if v == ActivationCode::SuccessfullyActivated as u8 => {
                Ok(ActivationCode::SuccessfullyActivated)
            }
            v if v == ActivationCode::ActivatedConfirmationRequired as u8 => {
                Ok(ActivationCode::ActivatedConfirmationRequired)
            }
            v => Err(Error::InvalidActivationCode { value: v }),
        }
    }
}

impl From<ActivationCode> for u8 {
    fn from(value: ActivationCode) -> Self {
        value as u8
    }
}
