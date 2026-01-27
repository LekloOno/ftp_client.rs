use crate::FtpResponseCode;
use crate::encode::{FtpEncoder, Result};
use crate::encode::Error;

/// Provides a simple UTF-8 encoder. </br>
/// 
/// # Notes
/// The implementation is very straightforward since rust String system is UTF-8 itself.
/// 
/// Most modern FTP server rely on either UTF-8 or ASCII, which directly binds to UTF-8. </br>
/// Thus, an UTF-8 encoder is a reasonnable default.
pub struct Utf8FtpEncoder;

impl FtpEncoder for Utf8FtpEncoder {
    fn encode<'a>(&self, string: &'a str) -> Result<&'a [u8]> {
        Ok(string.as_bytes())
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<&'a str> {
        str::from_utf8(bytes)
            .map_err(|_| Error::DecodingError)
    }

    fn crlf(&self) -> &'static [u8] { b"\r\n" }
    fn hyphen(&self) -> &'static [u8] { b"-" }
    
    fn response_code_len(&self) -> usize { 3 }
    
    fn decode_response_code(&self, bytes: &[u8]) -> Result<FtpResponseCode> {
        let sized_bytes = 
            TryInto::<&[u8; 3]>::try_into(bytes)
            .map_err(|_| Error::DecodingError)?
            .map(|u| u - b'0');

        Ok(FtpResponseCode::from_bytes(&sized_bytes))
    }
}