use thiserror::Error;

use crate::{FtpResponseCode, response::{FtpResponse, body_string}};

#[derive(Error, Debug)]
pub enum FtpErrorRes {
    #[error("331 UserNameOkPswdNeeded: {res}")]
    UserNameOkPswdNeeded {res: String}
}

impl From<FtpErrorRes> for crate::response::FtpResult {
    fn from(v: FtpErrorRes) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpErrorRes {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code.into() {
            331 => Some(Self::UserNameOkPswdNeeded { res: body_string(body) }),
            _ => None,
        }
    }
}