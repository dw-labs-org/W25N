use modular_bitfield::prelude::*;

#[bitfield]
pub struct Status1 {
    /// Status Register Protect-1
    pub srp1: bool,
    /// /WP Enable Bit
    pub wp_e: bool,
    /// Top/Bottom Protect Bits
    pub tb: bool,
    /// Block Protect Bits
    pub bp: B4,
    /// Status Register Protect-0
    pub srp0: bool,
}

impl From<Status1> for u8 {
    fn from(value: Status1) -> Self {
        value.bytes[0]
    }
}

#[bitfield]
pub struct Status2 {
    /// Hold Disable
    pub h_dis: bool,
    /// Output Dirver Strength
    pub osd: B2,
    /// Buffer Mode
    pub buf: bool,
    /// Enable ECC
    pub ecc_e: bool,
    /// Status Register-1 Lock
    pub sri_l: bool,
    /// Enter OTP Mode
    pub otp_e: bool,
    /// OTP Data Pages Lock
    pub otp_l: bool,
}

impl From<Status2> for u8 {
    fn from(value: Status2) -> Self {
        value.bytes[0]
    }
}

#[bitfield]
pub struct Status3 {
    /// Operation in Progress
    pub busy: bool,
    /// Write Enable Latch
    pub wel: bool,
    /// Erase Failure
    pub e_fail: bool,
    /// Program Failure
    pub p_fail: bool,
    /// ECC Status
    pub ecc: B2,
    reserved: B2,
}

impl From<Status3> for u8 {
    fn from(value: Status3) -> Self {
        value.bytes[0]
    }
}

pub struct Jedec {
    /// Jedec Manufacturer ID (0xEF)
    pub manufacturer: u8,
    /// Device ID
    pub device: u16,
}

impl From<[u8; 3]> for Jedec {
    fn from(value: [u8; 3]) -> Self {
        Jedec {
            manufacturer: value[0],
            device: ((value[1] as u16) << 8) + value[2] as u16,
        }
    }
}

impl Jedec {
    pub fn device_id(&self) -> Result<&'static str, &'static str> {
        match self.device {
            0xAA22 => Ok("W25N02KV"),
            _ => Err("Unkown"),
        }
    }
}
