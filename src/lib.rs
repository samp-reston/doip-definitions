// region:      --- Configs
#![cfg_attr(not(feature = "std"), no_std)] // Use no_std when the "std" feature is disabled
#![warn(clippy::pedantic)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![allow(clippy::module_name_repetitions)]
// endregion:      --- Configs

//! Diagnostics over Internet Protocol definition library
//!
//! This crate is built to act a definitive source for defintions of ISO-13400
//! and `DoIP` protocol development. Using all the data structures provided one
//! should be able to develop applications with certainty that they are abiding
//! by regulatory standards.
//!
//! Each `DoIP` message contains a minimum length of 8 bytes of which is the
//! [`header`], within this the header contains the length of the [`message`]
//! and what type of payload the message is.

// region:      --- Modules

mod doip_header;
mod doip_message;
mod doip_payload;

// -- Flatten

// -- Public Modules
/// Contains header related logic and data structures.
///
/// The `DoIP` header contains 4 items:
/// - Protocol Version
/// - Inverse Protocol Version
/// - Payload Type
/// - Payload Length
///
/// By default this library provides users with the option to use a different
/// protocol version than what is currently in use, this is for backwards and future
/// compatability.
///
/// The Inverse Protocol Version is the flipped byte of the protocol
/// version, this is to validate the packet.
///
/// The payload length is given in a `u32` but in this library is written in
/// `[u8; 4]` to remain close-to-network as possible.
///
/// This module contains the relevant structs, enums, and traits for developers
/// to create custom headers.
pub mod header {
    pub use crate::doip_header::DoipHeader;
    pub use automotive_diag::doip::{PayloadType, ProtocolVersion};
}

/// Contains message data structures and internal payload type dependant structures.
pub mod payload {
    pub use crate::doip_payload::DoipPayload;

    pub use automotive_diag::doip::{
        ActionCode, ActivationCode, ActivationType, DiagnosticAckCode, DiagnosticNackCode,
        NackCode, NodeType, PowerMode, SyncStatus,
    };

    pub use crate::doip_payload::alive_check_request::*;
    pub use crate::doip_payload::alive_check_response::*;
    pub use crate::doip_payload::diagnostic_message::*;
    pub use crate::doip_payload::diagnostic_message_ack::*;
    pub use crate::doip_payload::diagnostic_message_nack::*;
    pub use crate::doip_payload::entity_status_request::*;
    pub use crate::doip_payload::entity_status_response::*;
    pub use crate::doip_payload::generic_nack::*;
    pub use crate::doip_payload::power_information_request::*;
    pub use crate::doip_payload::power_information_response::*;
    pub use crate::doip_payload::routing_activation_request::*;
    pub use crate::doip_payload::routing_activation_response::*;
    pub use crate::doip_payload::vehicle_announcement_message::*;
    pub use crate::doip_payload::vehicle_identification_request::*;
    pub use crate::doip_payload::vehicle_identification_request_eid::*;
    pub use crate::doip_payload::vehicle_identification_request_vin::*;
}

/// The definitions found here are originally from Wireshark's repository. Wireshark
/// is a packet sniffing tool with an already supported `DoIP` and UDS packet disassembly
/// and so their definitions were lifted so to support this crate.
pub mod definitions;

/// Contains the implementations for the overarching `DoIP Message` structure.
pub mod message {
    pub use crate::doip_message::DoipMessage;
}

// endregion:      --- Modules

// Python bindings (only available when std is enabled)
#[cfg(feature = "std")]
#[cfg(any(not(test), rust_analyzer))]
mod bindings;

// Panic handler for `no_std` environments, but only when `std` is not enabled
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
