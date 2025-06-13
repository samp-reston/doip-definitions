use crate::error::{Error, Result};

/// The synchronisation status of the VIN and the GID for the entity
///
/// This gives the status that all `DoIP` entities have synchronised their information
/// about the VIN or GID of the vehicle.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncStatus {
    /// VIN/GID Synchronized
    VinGidSynchronized = 0x00,

    /// Reserved By ISO-13400 for byte value `01`
    ReservedByIso13400_01 = 0x01,

    /// Reserved By ISO-13400 for byte value `02`
    ReservedByIso13400_02 = 0x02,

    /// Reserved By ISO-13400 for byte value `03`
    ReservedByIso13400_03 = 0x03,

    /// Reserved By ISO-13400 for byte value `04`
    ReservedByIso13400_04 = 0x04,

    /// Reserved By ISO-13400 for byte value `05`
    ReservedByIso13400_05 = 0x05,

    /// Reserved By ISO-13400 for byte value `06`
    ReservedByIso13400_06 = 0x06,

    /// Reserved By ISO-13400 for byte value `07`
    ReservedByIso13400_07 = 0x07,

    /// Reserved By ISO-13400 for byte value `08`
    ReservedByIso13400_08 = 0x08,

    /// Reserved By ISO-13400 for byte value `09`
    ReservedByIso13400_09 = 0x09,

    /// Reserved By ISO-13400 for byte value `0A`
    ReservedByIso13400_0A = 0x0A,

    /// Reserved By ISO-13400 for byte value `0B`
    ReservedByIso13400_0B = 0x0B,

    /// Reserved By ISO-13400 for byte value `0C`
    ReservedByIso13400_0C = 0x0C,

    /// Reserved By ISO-13400 for byte value `0D`
    ReservedByIso13400_0D = 0x0D,

    /// Reserved By ISO-13400 for byte value `0E`
    ReservedByIso13400_0E = 0x0E,

    /// Reserved By ISO-13400 for byte value `0F`
    ReservedByIso13400_0F = 0x0F,

    /// VVIN/GID Not Synchronised
    VinGidNotSynchronised = 0x10,
}

impl TryFrom<&u8> for SyncStatus {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self> {
        let val = *value;

        match val {
            v if v == SyncStatus::VinGidSynchronized as u8 => Ok(SyncStatus::VinGidSynchronized),
            v if v == SyncStatus::ReservedByIso13400_01 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_01)
            }
            v if v == SyncStatus::ReservedByIso13400_02 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_02)
            }
            v if v == SyncStatus::ReservedByIso13400_03 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_03)
            }
            v if v == SyncStatus::ReservedByIso13400_04 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_04)
            }
            v if v == SyncStatus::ReservedByIso13400_05 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_05)
            }
            v if v == SyncStatus::ReservedByIso13400_06 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_06)
            }
            v if v == SyncStatus::ReservedByIso13400_07 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_07)
            }
            v if v == SyncStatus::ReservedByIso13400_08 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_08)
            }
            v if v == SyncStatus::ReservedByIso13400_09 as u8 => {
                Ok(SyncStatus::ReservedByIso13400_09)
            }
            v if v == SyncStatus::ReservedByIso13400_0A as u8 => {
                Ok(SyncStatus::ReservedByIso13400_0A)
            }
            v if v == SyncStatus::ReservedByIso13400_0B as u8 => {
                Ok(SyncStatus::ReservedByIso13400_0B)
            }
            v if v == SyncStatus::ReservedByIso13400_0C as u8 => {
                Ok(SyncStatus::ReservedByIso13400_0C)
            }
            v if v == SyncStatus::ReservedByIso13400_0D as u8 => {
                Ok(SyncStatus::ReservedByIso13400_0D)
            }
            v if v == SyncStatus::ReservedByIso13400_0E as u8 => {
                Ok(SyncStatus::ReservedByIso13400_0E)
            }
            v if v == SyncStatus::ReservedByIso13400_0F as u8 => {
                Ok(SyncStatus::ReservedByIso13400_0F)
            }
            v if v == SyncStatus::VinGidNotSynchronised as u8 => {
                Ok(SyncStatus::VinGidNotSynchronised)
            }
            v => Err(Error::InvalidSyncStatus { value: v }),
        }
    }
}

impl From<SyncStatus> for u8 {
    fn from(value: SyncStatus) -> Self {
        value as u8
    }
}
