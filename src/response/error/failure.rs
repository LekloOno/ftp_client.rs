use thiserror::Error;

use crate::response::FtpResponse;

#[derive(Error, Debug)]
pub enum FtpFailureRes {
    #[error("Some failure : {res}")]
    Default {res: String}
}

impl From<FtpFailureRes> for crate::response::FtpResult {
    fn from(v: FtpFailureRes) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpFailureRes {
    #[allow(unused)]
    fn from_bytes(code: u8, body: &[u8]) -> Option<Self> where Self: Sized {
        match code {
            _ => None
        }
    }
}