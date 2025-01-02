use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SyncStatus {
    VinGinSynchronized = 0x00,
    ReservedByIso13400_01 = 0x01,
    ReservedByIso13400_02 = 0x02,
    ReservedByIso13400_03 = 0x03,
    ReservedByIso13400_04 = 0x04,
    ReservedByIso13400_05 = 0x05,
    ReservedByIso13400_06 = 0x06,
    ReservedByIso13400_07 = 0x07,
    ReservedByIso13400_08 = 0x08,
    ReservedByIso13400_09 = 0x09,
    ReservedByIso13400_0A = 0x0A,
    ReservedByIso13400_0B = 0x0B,
    ReservedByIso13400_0C = 0x0C,
    ReservedByIso13400_0D = 0x0D,
    ReservedByIso13400_0E = 0x0E,
    ReservedByIso13400_0F = 0x0F,
    VinGinNotSynchronised = 0x10,
}

impl fmt::Display for SyncStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sync_strings = match self {
            SyncStatus::VinGinSynchronized => "VIN and/or GID are synchronized",
            SyncStatus::ReservedByIso13400_01 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_02 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_03 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_04 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_05 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_06 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_07 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_08 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_09 => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_0A => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_0B => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_0C => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_0D => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_0E => "Reserved by ISO 13400",
            SyncStatus::ReservedByIso13400_0F => "Reserved by ISO 13400",
            SyncStatus::VinGinNotSynchronised => "Incomplete: VIN and GID are NOT synchronized",
        };
        write!(f, "{}", sync_strings)
    }
}
