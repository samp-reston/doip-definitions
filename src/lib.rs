// region:      --- Configs
#![cfg_attr(not(feature = "std"), no_std)] // Use no_std when the "std" feature is disabled
#![cfg_attr(not(feature = "std"), no_alloc)] // Use no_alloc when the "std" feature is disabled
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
/// Error pertaining to functionality of trait implementations.
pub mod error;

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
    pub use crate::doip_header::payload_type::*;
    pub use crate::doip_header::version::*;
    pub use crate::doip_header::DoipHeader;
}

/// Contains message data structures and internal payload type dependant structures.
pub mod payload {
    pub use crate::doip_payload::action_code::*;
    pub use crate::doip_payload::activation_code::*;
    pub use crate::doip_payload::activation_type::*;
    pub use crate::doip_payload::diagnostic_ack::*;
    pub use crate::doip_payload::diagnostic_nack::*;
    pub use crate::doip_payload::nack_code::*;
    pub use crate::doip_payload::node_type::*;
    pub use crate::doip_payload::power_mode::*;
    pub use crate::doip_payload::sync_status::*;
    pub use crate::doip_payload::DoipPayload;

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

/// # `DoIP` Message Builder Library
///
/// This module provides functionality to construct Diagnostic over IP (`DoIP`) messages used in automotive diagnostics
/// in accordance with ISO 13400 standards.
///
/// It includes a builder pattern implementation (`DoipMessageBuilder`) that allows users to configure and construct
/// valid `DoipMessage` instances with appropriate protocol headers and payloads. The builder ensures that metadata
/// such as payload type and length are automatically derived from the provided payload content.
///
/// ## Features
///
/// - Typed and extensible `DoipPayload` variants for different message types
/// - Automatic header field population (e.g., inverse protocol version, payload length)
/// - Fluent, chainable builder interface
/// - Safe default values for rapid prototyping
///
/// ## Example Usage
///
/// ```rust
/// use doip_definitions::builder::DoipMessageBuilder;
/// use doip_definitions::payload::AliveCheckRequest;
/// use doip_definitions::payload::DoipPayload;
/// use doip_definitions::header::ProtocolVersion;
///
/// let message = DoipMessageBuilder::new()
///     .protocol_version(ProtocolVersion::Iso13400_2012)
///     .payload(DoipPayload::AliveCheckRequest(AliveCheckRequest {}))
///     .build();
/// ```
///
/// ## Message Types Supported
///
/// - Alive Check (Request/Response)
/// - Vehicle Identification (Generic, EID, VIN)
/// - Vehicle Announcement
/// - Routing Activation (Request/Response)
/// - Entity Status (Request/Response)
/// - Power Information (Request/Response)
/// - Diagnostic Messages (Standard, ACK, NACK)
///
/// ## Intended Usage
///
/// This library is designed for integration into vehicle diagnostics applications,
/// simulation tools, or any system that needs to generate or send valid `DoIP` messages over a network.
///
/// ## Compliance
///
/// The design adheres to the structural requirements defined in ISO 13400 and aims to maintain
/// compatibility with UDS-on-IP diagnostic stacks and OEM-specific `DoIP` implementations.
#[cfg(feature = "builder")]
pub mod builder;

// endregion:      --- Modules

// Python bindings (only available when python-bindings is enabled)
#[cfg(feature = "python-bindings")]
#[cfg(any(not(test), rust_analyzer))]
mod bindings;

// Panic handler for `no_std` environments, but only when `std` is not enabled
// #[cfg(not(feature = "std"))]
// #[panic_handler]
// fn panic(_info: &core::panic::PanicInfo) -> ! {
//     loop {}
// }
