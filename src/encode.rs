pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait FtpEncoder {
    fn encode<'a>(&self, string: &'a str) -> Result<&'a [u8]>;
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<&'a str>;
    
    fn crlf(&self) -> &'static [u8];

    fn hyphen(&self) -> &'static [u8];
    
    fn response_code_len(&self) -> usize;
    fn decode_response_code(&self, bytes: &[u8]) -> Result<u8>;
}