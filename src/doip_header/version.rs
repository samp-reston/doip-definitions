use crate::{
    definitions::{
        DEFAULT_VALUE, ISO13400_2010, ISO13400_2012, ISO13400_2019, ISO13400_2019_AMD1,
        RESERVED_VER,
    },
    error::Error,
};

/// Avaiable version of the `DoIP` protocol as per ISO-13400.
///
/// Maps to `u8` values for avaiable `DoIP` protocols which are supported by this
/// crates and ISO-13400.
#[cfg_attr(feature = "python-bindings", pyo3::pyclass(eq, eq_int))]
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum ProtocolVersion {
    /// Reserved Version
    ReservedVer = RESERVED_VER,

    /// `DoIP` Payload Version: ISO-13400 2010 Version
    Iso13400_2010 = ISO13400_2010,

    /// `DoIP` Payload Version: ISO-13400 2012 Version
    Iso13400_2012 = ISO13400_2012,

    /// `DoIP` Payload Version: ISO-13400 2019 Version
    Iso13400_2019 = ISO13400_2019,

    /// `DoIP` Payload Version: ISO-13400 `2019_AMD1` Version
    Iso13400_2019Amd1 = ISO13400_2019_AMD1,

    /// `DoIP` Payload Version: Default Version
    DefaultValue = DEFAULT_VALUE,
}

impl TryFrom<&u8> for ProtocolVersion {
    type Error = Error;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        let val = *value;

        match val {
            v if v == ProtocolVersion::ReservedVer as u8 => Ok(ProtocolVersion::ReservedVer),
            v if v == ProtocolVersion::Iso13400_2010 as u8 => Ok(ProtocolVersion::Iso13400_2010),
            v if v == ProtocolVersion::Iso13400_2012 as u8 => Ok(ProtocolVersion::Iso13400_2012),
            v if v == ProtocolVersion::Iso13400_2019 as u8 => Ok(ProtocolVersion::Iso13400_2019),
            v if v == ProtocolVersion::Iso13400_2019Amd1 as u8 => {
                Ok(ProtocolVersion::Iso13400_2019Amd1)
            }
            v if v == ProtocolVersion::DefaultValue as u8 => Ok(ProtocolVersion::DefaultValue),
            v => Err(Error::InvalidProtocolVersion { value: v }),
        }
    }
}

impl From<ProtocolVersion> for u8 {
    fn from(value: ProtocolVersion) -> Self {
        value as u8
    }
}
