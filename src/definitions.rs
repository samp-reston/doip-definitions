
pub const DOIP_PORT: usize = 13400;
pub const DOIP_TLS_PORT: usize = 3496;

pub const DOIP_GENERIC_NACK: u16 = 0x0000;
pub const DOIP_VEHICLE_IDENTIFICATION_REQ: u16 = 0x0001;
pub const DOIP_VEHICLE_IDENTIFICATION_REQ_EID: u16 = 0x0002;
pub const DOIP_VEHICLE_IDENTIFICATION_REQ_VIN: u16 = 0x0003;
pub const DOIP_VEHICLE_ANNOUNCEMENT_MESSAGE: u16 = 0x0004;
pub const DOIP_ROUTING_ACTIVATION_REQUEST: u16 = 0x0005;
pub const DOIP_ROUTING_ACTIVATION_RESPONSE: u16 = 0x0006;
pub const DOIP_ALIVE_CHECK_REQUEST: u16 = 0x0007;
pub const DOIP_ALIVE_CHECK_RESPONSE: u16 = 0x0008;
pub const DOIP_ENTITY_STATUS_REQUEST: u16 = 0x4001;
pub const DOIP_ENTITY_STATUS_RESPONSE: u16 = 0x4002;
pub const DOIP_POWER_INFORMATION_REQUEST: u16 = 0x4003;
pub const DOIP_POWER_INFORMATION_RESPONSE: u16 = 0x4004;
pub const DOIP_DIAGNOSTIC_MESSAGE: u16 = 0x8001;
pub const DOIP_DIAGNOSTIC_MESSAGE_ACK: u16 = 0x8002;
pub const DOIP_DIAGNOSTIC_MESSAGE_NACK: u16 = 0x8003;

// DoIP Header //
pub const DOIP_VERSION_OFFSET: usize = 0;
pub const DOIP_VERSION_LEN: usize = 1;
pub const DOIP_INV_VERSION_OFFSET: usize = DOIP_VERSION_OFFSET + DOIP_VERSION_LEN;
pub const DOIP_INV_VERSION_LEN: usize = 1;
pub const DOIP_TYPE_OFFSET: usize = DOIP_INV_VERSION_OFFSET + DOIP_INV_VERSION_LEN;
pub const DOIP_TYPE_LEN: usize = 2;
pub const DOIP_LENGTH_OFFSET: usize = DOIP_TYPE_OFFSET + DOIP_TYPE_LEN;
pub const DOIP_LENGTH_LEN: usize = 4;
pub const DOIP_HEADER_LEN: usize = DOIP_LENGTH_OFFSET + DOIP_LENGTH_LEN;

// DoIP Payload Version //
pub const RESERVED_VER: u8 = 0x00;
pub const ISO13400_2010: u8 = 0x01;
pub const ISO13400_2012: u8 = 0x02;
pub const ISO13400_2019: u8 = 0x03;
pub const ISO13400_2019_AMD1: u8 = 0x04;
pub const DEFAULT_VALUE: u8 = 0xFF;

// Generic NACK //
pub const DOIP_GENERIC_NACK_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_GENERIC_NACK_LEN: usize = 1;

// Common //
pub const DOIP_COMMON_VIN_LEN: usize = 17;
pub const DOIP_COMMON_EID_LEN: usize = 6;

// Vehicle identification request //
pub const DOIP_VEHICLE_IDENTIFICATION_EID_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_VEHICLE_IDENTIFICATION_VIN_OFFSET: usize = DOIP_HEADER_LEN;

// Routing activation request //
pub const DOIP_ROUTING_ACTIVATION_REQ_SRC_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN: usize = 2;
pub const DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET: usize =
    DOIP_ROUTING_ACTIVATION_REQ_SRC_OFFSET + DOIP_ROUTING_ACTIVATION_REQ_SRC_LEN;
pub const DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V1: usize = 2;
pub const DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2: usize = 1;
pub const DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V1: usize =
    DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET + DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V1;
pub const DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V2: usize =
    DOIP_ROUTING_ACTIVATION_REQ_TYPE_OFFSET + DOIP_ROUTING_ACTIVATION_REQ_TYPE_LEN_V2;
pub const DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN: usize = 4;
pub const DOIP_ROUTING_ACTIVATION_REQ_OEM_OFFSET_V1: usize =
    DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V1 + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN;
pub const DOIP_ROUTING_ACTIVATION_REQ_OEM_OFFSET_V2: usize =
    DOIP_ROUTING_ACTIVATION_REQ_ISO_OFFSET_V2 + DOIP_ROUTING_ACTIVATION_REQ_ISO_LEN;
pub const DOIP_ROUTING_ACTIVATION_REQ_OEM_LEN: usize = 4;

// Routing activation response //
pub const DOIP_ROUTING_ACTIVATION_RES_TESTER_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN: usize = 2;
pub const DOIP_ROUTING_ACTIVATION_RES_ENTITY_OFFSET: usize =
    DOIP_ROUTING_ACTIVATION_RES_TESTER_OFFSET + DOIP_ROUTING_ACTIVATION_RES_TESTER_LEN;
pub const DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN: usize = 2;
pub const DOIP_ROUTING_ACTIVATION_RES_CODE_OFFSET: usize =
    DOIP_ROUTING_ACTIVATION_RES_ENTITY_OFFSET + DOIP_ROUTING_ACTIVATION_RES_ENTITY_LEN;
pub const DOIP_ROUTING_ACTIVATION_RES_CODE_LEN: usize = 1;
pub const DOIP_ROUTING_ACTIVATION_RES_ISO_OFFSET: usize =
    DOIP_ROUTING_ACTIVATION_RES_CODE_OFFSET + DOIP_ROUTING_ACTIVATION_RES_CODE_LEN;
pub const DOIP_ROUTING_ACTIVATION_RES_ISO_LEN: usize = 4;
pub const DOIP_ROUTING_ACTIVATION_RES_OEM_OFFSET: usize =
    DOIP_ROUTING_ACTIVATION_RES_ISO_OFFSET + DOIP_ROUTING_ACTIVATION_RES_ISO_LEN;
pub const DOIP_ROUTING_ACTIVATION_RES_OEM_LEN: usize = 4;

// Vehicle announcement message //
pub const DOIP_VEHICLE_ANNOUNCEMENT_VIN_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_OFFSET: usize =
    DOIP_VEHICLE_ANNOUNCEMENT_VIN_OFFSET + DOIP_COMMON_VIN_LEN;
pub const DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_LEN: usize = 2;
pub const DOIP_VEHICLE_ANNOUNCEMENT_EID_OFFSET: usize =
    DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_OFFSET + DOIP_VEHICLE_ANNOUNCEMENT_ADDRESS_LEN;
pub const DOIP_VEHICLE_ANNOUNCEMENT_GID_OFFSET: usize =
    DOIP_VEHICLE_ANNOUNCEMENT_EID_OFFSET + DOIP_COMMON_EID_LEN;
pub const DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN: usize = 6;
pub const DOIP_VEHICLE_ANNOUNCEMENT_ACTION_OFFSET: usize =
    DOIP_VEHICLE_ANNOUNCEMENT_GID_OFFSET + DOIP_VEHICLE_ANNOUNCEMENT_GID_LEN;
pub const DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN: usize = 1;
pub const DOIP_VEHICLE_ANNOUNCEMENT_SYNC_OFFSET: usize =
    DOIP_VEHICLE_ANNOUNCEMENT_ACTION_OFFSET + DOIP_VEHICLE_ANNOUNCEMENT_ACTION_LEN;
pub const DOIP_VEHICLE_ANNOUNCEMENT_SYNC_LEN: usize = 1;

// Alive check response //
pub const DOIP_ALIVE_CHECK_RESPONSE_SOURCE_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_ALIVE_CHECK_RESPONSE_SOURCE_LEN: usize = 2;

// Entity status response //
pub const DOIP_ENTITY_STATUS_RESPONSE_NODE_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN: usize = 1;
pub const DOIP_ENTITY_STATUS_RESPONSE_MCTS_OFFSET: usize =
    DOIP_ENTITY_STATUS_RESPONSE_NODE_OFFSET + DOIP_ENTITY_STATUS_RESPONSE_NODE_LEN;
pub const DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN: usize = 1;
pub const DOIP_ENTITY_STATUS_RESPONSE_NCTS_OFFSET: usize =
    DOIP_ENTITY_STATUS_RESPONSE_MCTS_OFFSET + DOIP_ENTITY_STATUS_RESPONSE_MCTS_LEN;
pub const DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN: usize = 1;
pub const DOIP_ENTITY_STATUS_RESPONSE_MDS_OFFSET: usize =
    DOIP_ENTITY_STATUS_RESPONSE_NCTS_OFFSET + DOIP_ENTITY_STATUS_RESPONSE_NCTS_LEN;
pub const DOIP_ENTITY_STATUS_RESPONSE_MDS_LEN: usize = 4;

// Diagnostic power mode information response //
pub const DOIP_POWER_MODE_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_POWER_MODE_LEN: usize = 1;

// Common //
pub const DOIP_DIAG_COMMON_SOURCE_OFFSET: usize = DOIP_HEADER_LEN;
pub const DOIP_DIAG_COMMON_SOURCE_LEN: usize = 2;
pub const DOIP_DIAG_COMMON_TARGET_OFFSET: usize =
    DOIP_DIAG_COMMON_SOURCE_OFFSET + DOIP_DIAG_COMMON_SOURCE_LEN;
pub const DOIP_DIAG_COMMON_TARGET_LEN: usize = 2;

// Diagnostic message //
pub const DOIP_DIAG_MESSAGE_DATA_OFFSET: usize =
    DOIP_DIAG_COMMON_TARGET_OFFSET + DOIP_DIAG_COMMON_TARGET_LEN;

// Diagnostic message ACK //
pub const DOIP_DIAG_MESSAGE_ACK_CODE_OFFSET: usize =
    DOIP_DIAG_COMMON_TARGET_OFFSET + DOIP_DIAG_COMMON_TARGET_LEN;
pub const DOIP_DIAG_MESSAGE_ACK_CODE_LEN: usize = 1;
pub const DOIP_DIAG_MESSAGE_ACK_PREVIOUS_OFFSET: usize =
    DOIP_DIAG_MESSAGE_ACK_CODE_OFFSET + DOIP_DIAG_MESSAGE_ACK_CODE_LEN;

// Diagnostic message NACK //
pub const DOIP_DIAG_MESSAGE_NACK_CODE_OFFSET: usize =
    DOIP_DIAG_COMMON_TARGET_OFFSET + DOIP_DIAG_COMMON_TARGET_LEN;
pub const DOIP_DIAG_MESSAGE_NACK_CODE_LEN: usize = 1;
pub const DOIP_DIAG_MESSAGE_NACK_PREVIOUS_OFFSET: usize =
    DOIP_DIAG_MESSAGE_NACK_CODE_OFFSET + DOIP_DIAG_MESSAGE_NACK_CODE_LEN;
