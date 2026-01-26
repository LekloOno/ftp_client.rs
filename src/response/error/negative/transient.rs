use thiserror::Error;

use crate::{FtpResponseCode, response::FtpResponse};

#[derive(Debug, Error)]
pub enum FtpTransientError {

}

impl From<FtpTransientError> for crate::response::FtpResult {
    fn from(v: FtpTransientError) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpTransientError {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        None
    }
}