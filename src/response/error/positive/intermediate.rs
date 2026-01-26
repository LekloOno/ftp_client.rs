use thiserror::Error;

use crate::{FtpResponseCode, response::{FtpResponse, body_string}};

#[derive(Debug, Error)]
pub enum FtpIntermediateReply {
    #[error("331 UserNameOkPswdNeeded: {res}")]
    UserNameOkPswdNeeded {res: String}
}

impl From<FtpIntermediateReply> for crate::response::FtpResult {
    fn from(v: FtpIntermediateReply) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpIntermediateReply {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code.into() {
            331 => Some(Self::UserNameOkPswdNeeded { res: body_string(body) }),
            _ => None,
        }
    }
}