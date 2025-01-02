use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DiagnosticNackCode {
    ReservedByIso13400_00 = 0x00,
    ReservedByIso13400_01 = 0x01,
    InvalidSourceAddress = 0x02,
    UnknownTargetAddress = 0x03,
    DiagnosticMessageTooLarge = 0x04,
    OutOfMemory = 0x05,
    TargetUnreachable = 0x06,
    UnknownNetwork = 0x07,
    TransportProtocolError = 0x08,
}

impl fmt::Display for DiagnosticNackCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let diag_strings = match self {
            DiagnosticNackCode::ReservedByIso13400_00 => "Reserved by ISO 13400",
            DiagnosticNackCode::ReservedByIso13400_01 => "Reserved by ISO 13400",
            DiagnosticNackCode::InvalidSourceAddress => "Invalid source address",
            DiagnosticNackCode::UnknownTargetAddress => "Unknown target address",
            DiagnosticNackCode::DiagnosticMessageTooLarge => "Diagnostic message too large",
            DiagnosticNackCode::OutOfMemory => "Out of memory",
            DiagnosticNackCode::TargetUnreachable => "Target unreachable",
            DiagnosticNackCode::UnknownNetwork => "Unknown network",
            DiagnosticNackCode::TransportProtocolError => "Transport protocol error",
        };
        write!(f, "{}", diag_strings)
    }
}
