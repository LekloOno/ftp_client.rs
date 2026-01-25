use crate::encode::{FtpEncoder, Result};
use crate::encode::Error;

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
    
    fn response_code_len(&self) -> usize {
        3
    }
    
    fn decode_response_code(&self, bytes: &[u8]) -> Result<u8> {
        if bytes.len() != 3 {
            return Err(Error::DecodingError);
        }
        
        let mut value = 0;
        for d in bytes {
            value = value * 10 + (d - b'0');
        }

        Ok(value)
    }
}