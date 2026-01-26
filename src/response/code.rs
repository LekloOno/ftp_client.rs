#[derive(Debug, Clone)]
pub struct FtpResponseCode(u16);

impl FtpResponseCode {
    /// Builds an FtpResponseCode from a raw bytes buffer.
    ///
    /// # Notes
    /// The buffer is expected to be big-endian.
    /// 
    /// # Arguments
    /// * bytes - a big endian buffer of 3 bytes representing the code digits.
    pub fn from_bytes(bytes: &[u8; 3]) -> Self {
        Self::new(bytes[0], bytes[1], bytes[2])
    }

    pub fn new(outcome: u8, kind: u8, details: u8) -> Self {
        let sh_outcome = (outcome as u16) << 8;
        let sh_kind = (kind as u16) << 4;
        let sh_details = details as u16;

        Self(sh_outcome | sh_kind | sh_details)
    }

    pub fn outcome(&self) -> u8 {
        ((self.0 >> 8) & 0xf) as u8
    }

    pub fn kind(&self) -> u8 {
        ((self.0 >> 4) & 0xf) as u8
    }

    pub fn details(&self) -> u8 {
        (self.0 as u8) & 0xf
    }
}

impl Into<u16> for FtpResponseCode {
    fn into(self) -> u16 {
        ((self.0 >> 8) & 0xf) * 100
        + ((self.0 >> 4) & 0xf) * 10
        + (self.0 & 0xf)
    }
} 

impl Into<u16> for &FtpResponseCode {
    fn into(self) -> u16 {
        ((self.0 >> 8) & 0xf) * 100
        + ((self.0 >> 4) & 0xf) * 10
        + (self.0 & 0xf)
    }
}

impl std::fmt::Display for FtpResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<u16>::into(self))
    }
}