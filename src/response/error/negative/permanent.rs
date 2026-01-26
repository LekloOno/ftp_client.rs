use thiserror::Error;

use crate::{FtpResponseCode, response::{FtpResponse, body_string}};

#[derive(Debug, Error)]
pub enum FtpPermanentError {
    #[error("530 NotLoggedIn : {res}")]
    NotLoggedIn {res: String},
}

impl From<FtpPermanentError> for crate::response::FtpResult {
    fn from(v: FtpPermanentError) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpPermanentError {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code.into() {
            530 => Some(Self::NotLoggedIn { res: body_string(body) }),
            _ => None,
        }
    }
}