use thiserror::Error;

use crate::{FtpResponseCode, response::FtpResponse};
use crate::response::body_string;

#[derive(Error, Debug)]
pub enum FtpFailureRes {
    #[error("530 NotLoggedIn : {res}")]
    NotLoggedIn {res: String}
}

impl From<FtpFailureRes> for crate::response::FtpResult {
    fn from(v: FtpFailureRes) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpFailureRes {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code.into() {
            530 => Some(Self::NotLoggedIn { res: body_string(body) }),
            _ => None
        }
    }
}