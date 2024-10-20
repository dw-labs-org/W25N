pub const RESET: u8 = 0xFF;
pub const JEDEC: u8 = 0x9F;
pub const READ_REG: u8 = 0x05;
pub const WRITE_REG: u8 = 0x01;
pub const WRITE_ENABLE: u8 = 0x06;
pub const WRITE_DISABLE: u8 = 0x04;
pub const BLOCK_ERASE: u8 = 0xD8;
pub const PROGRAM_DATA_LOAD: u8 = 0x02;
pub const RANDOM_PROGRAM_DATA_LOAD: u8 = 0x84;
pub const QUAD_PROGRAM_DATA_LOAD: u8 = 0x32;
pub const RANDOM_QUAD_PROGRAM_DATA_LOAD: u8 = 0x34;
pub const PROGRAM_EXECUTE: u8 = 0x10;
pub const PAGE_DATA_READ: u8 = 0x13;
pub const READ: u8 = 0x03;
pub const FAST_READ: u8 = 0x0B;
pub const FAST_READ_4_BYTE_ADDRESS: u8 = 0x0C;
pub const FAST_READ_DUAL_OUTPUT: u8 = 0x3B;
pub const FAST_READ_DUAL_OUTPUT_4_BYTE_ADDRESS: u8 = 0x3C;
pub const FAST_READ_QUAD_OUTPUT: u8 = 0x6B;
pub const FAST_READ_QUAD_OUTPUT_4_BYTE_ADDRESS: u8 = 0x6C;
pub const FAST_READ_DUAL_IO: u8 = 0xBB;
pub const FAST_READ_DUAL_IO_4_BYTE_ADDRESS: u8 = 0xBC;
pub const FAST_READ_QUAD_IO: u8 = 0xEB;
pub const FAST_READ_QUAD_IO_4_BYTE_ADDRESS: u8 = 0xEC;
pub const DEEP_POWER_DOWN: u8 = 0xB9;
pub const RELEASE_POWER_DOWN: u8 = 0xAB;
pub const ENABLE_RESET: u8 = 0x66;
pub const RESET_DEVICE: u8 = 0x99;

pub const STATUS_REGISTER_1: u8 = 0xA0;
pub const STATUS_REGISTER_2: u8 = 0xB0;
pub const STATUS_REGISTER_3: u8 = 0xC0;
