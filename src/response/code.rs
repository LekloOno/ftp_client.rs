#[derive(Clone)]
pub struct FtpResponseCode(u16);

impl std::fmt::Debug for FtpResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FtpResponseCode({})", Into::<u16>::into(self))
    }
}

/// Wraps an FtpResponseCode under a bytes-packed structure
/// with an explicit interface.
/// 
/// # Notes
/// We could eventually make FtpResponseCode restricted to lifetime `'a`
/// of the bytes it is built from. To do later.
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

    /// The first digit (`x`) of the response code `Xyz`:  <br/>
    /// <br/>
    /// `1` - Positive Preliminary reply <br/>
    /// `2` - Positive Completion reply <br/>
    /// `3` - Positive Intermediate reply <br/>
    /// `4` - Transient Negative Completion reply <br/>
    /// `5` - Permanent Negative Completion reply <br/>
    pub fn outcome(&self) -> u8 {
        ((self.0 >> 8) & 0xf) as u8
    }

    /// The second digit (`y`) of the response code `xYz`: <br/>
    /// <br/>
    /// `0` - Syntax <br/>
    /// `1` - Information <br/>
    /// `2` - Connections <br/>
    /// `3` - Authentication <br/>
    /// `4` - Not defined <br/>
    /// `5` - File system <br/> 
    pub fn kind(&self) -> u8 {
        ((self.0 >> 4) & 0xf) as u8
    }

    /// The third digit (`z`) of the response code `xyZ`.
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