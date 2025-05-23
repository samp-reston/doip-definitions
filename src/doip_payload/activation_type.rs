/// Used in Routing Activation Request to request specific routing types.
///
/// Used to customise the routing type requested from the `DoIP` entity for different
/// scenarios.
#[cfg_attr(feature = "std", pyo3::pyclass(eq, eq_int))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActivationType {
    /// Default
    Default = 0x00,

    /// WWH-OBD
    WwhObd = 0x01,

    /// Central Security
    CentralSecurity = 0x02,
}
