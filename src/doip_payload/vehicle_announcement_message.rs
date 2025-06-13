use crate::{
    definitions::{
        DOIP_COMMON_EID_LEN, DOIP_COMMON_VIN_LEN, DOIP_DIAG_COMMON_SOURCE_LEN,
        DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN,
    },
    error::{Error, Result},
    payload::{ActionCode, SyncStatus},
};

/// Announcement from a `DoIP` entity in response to a
/// `VehicleIdentificationRequest[EID/VIN]`.
///
/// The positive response to a `VehicleIdentificationRequest[EID/VIN]` request
/// containing the vehicle information from the `DoIP` entity.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VehicleAnnouncementMessage {
    /// Vehicle Identification Number
    pub vin: [u8; DOIP_COMMON_VIN_LEN],

    /// Logical address of responding entity
    pub logical_address: [u8; DOIP_DIAG_COMMON_SOURCE_LEN],

    /// Entity Identification
    pub eid: [u8; DOIP_COMMON_EID_LEN],

    /// Group Identification
    pub gid: [u8; DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN],

    /// Further actions required
    pub further_action: ActionCode,

    /// Status of VIN/GID Synchronisation
    pub vin_gid_sync: Option<SyncStatus>,
}

impl From<VehicleAnnouncementMessage>
    for [u8; DOIP_COMMON_VIN_LEN
        + DOIP_DIAG_COMMON_SOURCE_LEN
        + DOIP_COMMON_EID_LEN
        + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
        + 2]
{
    fn from(value: VehicleAnnouncementMessage) -> Self {
        let mut buffer = [0u8; DOIP_COMMON_VIN_LEN
            + DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_COMMON_EID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
            + 2];

        let mut offset = 0;

        buffer[offset..offset + DOIP_COMMON_VIN_LEN].copy_from_slice(&value.vin);
        offset += DOIP_COMMON_VIN_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN]
            .copy_from_slice(&value.logical_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_COMMON_EID_LEN].copy_from_slice(&value.eid);
        offset += DOIP_COMMON_EID_LEN;

        buffer[offset..offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN].copy_from_slice(&value.gid);
        offset += DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;

        buffer[offset] = value.further_action.into();
        offset += 1;

        buffer[offset] = value.vin_gid_sync.map_or(0, Into::into);

        buffer
    }
}

impl From<VehicleAnnouncementMessage>
    for [u8; DOIP_COMMON_VIN_LEN
        + DOIP_DIAG_COMMON_SOURCE_LEN
        + DOIP_COMMON_EID_LEN
        + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
        + 1]
{
    fn from(value: VehicleAnnouncementMessage) -> Self {
        let mut buffer = [0u8; DOIP_COMMON_VIN_LEN
            + DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_COMMON_EID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
            + 1];

        let mut offset = 0;

        buffer[offset..offset + DOIP_COMMON_VIN_LEN].copy_from_slice(&value.vin);
        offset += DOIP_COMMON_VIN_LEN;

        buffer[offset..offset + DOIP_DIAG_COMMON_SOURCE_LEN]
            .copy_from_slice(&value.logical_address);
        offset += DOIP_DIAG_COMMON_SOURCE_LEN;

        buffer[offset..offset + DOIP_COMMON_EID_LEN].copy_from_slice(&value.eid);
        offset += DOIP_COMMON_EID_LEN;

        buffer[offset..offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN].copy_from_slice(&value.gid);
        offset += DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;

        buffer[offset] = value.further_action.into();

        buffer
    }
}

impl TryFrom<&[u8]> for VehicleAnnouncementMessage {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        // Calculate minimum lengths
        let min_len_with_sync = DOIP_COMMON_VIN_LEN
            + DOIP_DIAG_COMMON_SOURCE_LEN
            + DOIP_COMMON_EID_LEN
            + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN
            + 2; // further_action + vin_gid_sync

        let min_len_without_sync = min_len_with_sync - 1; // without vin_gid_sync

        if value.len() < min_len_without_sync {
            return Err(Error::SliceTooSmall);
        }

        let mut offset = 0;

        // Extract VIN
        let vin_end = offset + DOIP_COMMON_VIN_LEN;
        let vin = value
            .get(offset..vin_end)
            .ok_or(Error::OutOfBounds {
                source: "VehicleAnnouncementMessage",
                variable: "Vin",
            })?
            .try_into()?;
        offset = vin_end;

        // Extract logical_address
        let logical_end = offset + DOIP_DIAG_COMMON_SOURCE_LEN;
        let logical_address = value
            .get(offset..logical_end)
            .ok_or(Error::OutOfBounds {
                source: "VehicleAnnouncementMessage",
                variable: "Logical Address",
            })?
            .try_into()
            .unwrap();
        offset = logical_end;

        // Extract eid
        let eid_end = offset + DOIP_COMMON_EID_LEN;
        let eid = value
            .get(offset..eid_end)
            .ok_or(Error::OutOfBounds {
                source: "VehicleAnnouncementMessage",
                variable: "EID",
            })?
            .try_into()
            .unwrap();
        offset = eid_end;

        // Extract gid
        let gid_end = offset + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;
        let gid = value
            .get(offset..gid_end)
            .ok_or(Error::OutOfBounds {
                source: "VehicleAnnouncementMessage",
                variable: "GID",
            })?
            .try_into()
            .unwrap();
        offset = gid_end;

        // Extract further_action
        let further_action_byte = value.get(offset).ok_or(Error::OutOfBounds {
            source: "VehicleAnnouncementMessage",
            variable: "Further Actions",
        })?;
        offset += 1;
        let further_action = further_action_byte.try_into()?;

        // Extract optional vin_gid_sync if present
        let vin_gid_sync: Option<SyncStatus> = if value.len() > offset {
            let val = &value[offset];
            let status: SyncStatus = val.try_into()?;
            Some(status)
        } else {
            None
        };

        Ok(VehicleAnnouncementMessage {
            vin,
            logical_address,
            eid,
            gid,
            further_action,
            vin_gid_sync,
        })
    }
}
