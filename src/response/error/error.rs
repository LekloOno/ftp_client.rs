use thiserror::Error;

use crate::{FtpResponseCode, response::FtpResponse};

#[derive(Error, Debug)]
pub enum FtpErrorRes {
    #[error("Some error : {res}")]
    Default {res: String}
}

impl From<FtpErrorRes> for crate::response::FtpResult {
    fn from(v: FtpErrorRes) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpErrorRes {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code {
            _ => None,
        }
    }
}