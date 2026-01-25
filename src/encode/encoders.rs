use crate::encode::{FtpEncoder, Result};
use crate::encode::error::Error;

pub struct Utf8FtpEncoder;

impl FtpEncoder for Utf8FtpEncoder {
    fn encode<'a>(&self, string: &'a str) -> Result<&'a [u8]> {
        Ok(string.as_bytes())
    }

    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<&'a str> {
        str::from_utf8(bytes)
            .map_err(Error::DecodingError)
    }

    fn crlf(&self) -> &'static [u8] { b"\r\n" }
    fn hyphen(&self) -> &'static [u8] { b"-" }

    fn read_resp_code(&self, stream: &mut dyn std::io::Read) -> std::io::Result<u8> {
        let mut code_bytes = [0u8; 3];
        
        stream
            .read_exact(&mut code_bytes)
            .map_err(Error::DecodingError);

        let mut value = 0;
        for d in code_bytes {
            value = value * 10 + d.parse::<u8>().unwrap();
        }

        Ok(value)
    }
}