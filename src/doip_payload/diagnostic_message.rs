use crate::definitions::{DOIP_DIAG_COMMON_SOURCE_LEN, DOIP_DIAG_COMMON_TARGET_LEN};
use crate::error::{Error, Result};

/// A UDS Message to a specific target address.
///
/// `DiagnosticMessage` is the most utilised payload type due to the amount of actions
/// a diagnostic tester can do using the UDS protocol. This crate will not handle the UDS
/// protocol however, one will be developed to enhance developer tooling.
#[cfg(not(feature = "std"))]
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct DiagnosticMessage<const N: usize> {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// Message containing the UDS protocol message
    pub message: [u8; N],
}

#[cfg(not(feature = "std"))]
impl<const N: usize> DiagnosticMessage<N> {
    /// Returns the size of the `DiagnosticMessage` payload in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + self.message.len()
    }
}

#[cfg(not(feature = "std"))]
impl<const N: usize> TryFrom<&[u8]> for DiagnosticMessage<N> {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let source_address = value
            .get(offset..DOIP_DIAG_COMMON_SOURCE_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessage",
                variable: "Source Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        let target_address = value
            .get(offset..offset + DOIP_DIAG_COMMON_TARGET_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessage",
                variable: "Target Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        let message: [u8; N] = value
            .get(offset..)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessage",
                variable: "Message",
            })?
            .try_into()?;

        Ok(DiagnosticMessage {
            source_address,
            target_address,
            message,
        })
    }
}

#[cfg(not(feature = "std"))]
impl<const N: usize> From<DiagnosticMessage<N>> for [u8; N] {
    fn from(value: DiagnosticMessage<N>) -> Self {
        let mut buffer = [0u8; N];

        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&value.source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN].copy_from_slice(&value.target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset..].copy_from_slice(&value.message);

        buffer
    }
}

/// A UDS Message to a specific target address.
///
/// `DiagnosticMessage` is the most utilised payload type due to the amount of actions
/// a diagnostic tester can do using the UDS protocol. This crate will not handle the UDS
/// protocol however, one will be developed to enhance developer tooling.
#[cfg(feature = "std")]
#[derive(Clone, Debug, PartialEq)]
pub struct DiagnosticMessage {
    /// The source address of the responding `DoIP` Entity
    pub source_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// The target address of the requesting `DoIP` Entity
    pub target_address: [u8; DOIP_DIAG_COMMON_TARGET_LEN],

    /// Message containing the UDS protocol message
    pub message: Vec<u8>,
}

#[cfg(feature = "std")]
impl DiagnosticMessage {
    /// Returns the size of the `DiagnosticMessage` payload in bytes.
    #[must_use]
    pub fn size(&self) -> usize {
        DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + self.message.len()
    }
}

#[cfg(feature = "std")]
impl From<DiagnosticMessage> for Vec<u8> {
    fn from(value: DiagnosticMessage) -> Self {
        let mut buffer =
            vec![
                0u8;
                DOIP_DIAG_COMMON_SOURCE_LEN + DOIP_DIAG_COMMON_TARGET_LEN + value.message.len()
            ];

        let mut offset = 0;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN].copy_from_slice(&value.source_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_TARGET_LEN].copy_from_slice(&value.target_address);
        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        buffer[offset..].copy_from_slice(&value.message);

        buffer
    }
}

#[cfg(feature = "std")]
impl TryFrom<&[u8]> for DiagnosticMessage {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        let mut offset = 0;

        let source_address = value
            .get(offset..DOIP_DIAG_COMMON_SOURCE_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessage",
                variable: "Source Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        let target_address = value
            .get(offset..offset + DOIP_DIAG_COMMON_TARGET_LEN)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessage",
                variable: "Target Address",
            })?
            .try_into()?;

        offset += DOIP_DIAG_COMMON_TARGET_LEN;

        let message = value
            .get(offset..)
            .ok_or(Error::OutOfBounds {
                source: "DiagnosticMessage",
                variable: "Message",
            })?
            .to_vec();

        Ok(DiagnosticMessage {
            source_address,
            target_address,
            message,
        })
    }
}
