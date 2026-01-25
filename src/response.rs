use std::convert::Infallible;
use std::str::FromStr;

pub enum FtpResponse {
    PLACEHOLDER(Vec<u8>),
}

impl FtpResponse {
    pub fn new(_code: u8, body: &[u8]) -> Self {
        FtpResponse::PLACEHOLDER(body.to_vec())
    }
}

impl FromStr for FtpResponse {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FtpResponse::PLACEHOLDER(s.as_bytes().to_vec()))
    }
}