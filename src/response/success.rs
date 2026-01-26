use crate::{FtpResponseCode, response::FtpResponse};
use crate::response::body_string;

#[derive(Debug)]
pub enum FtpSuccessRes {
    PathNameCreated {pathname: String},
    ServiceReady { msg: String }
}

impl From<FtpSuccessRes> for super::FtpResult {
    fn from(v: FtpSuccessRes) -> Self {
        Self::Ok(v)
    }
}

impl FtpResponse for FtpSuccessRes {
    /// # Panics
    /// The parsing may panic if the body can't be parsed to valid utf8.
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code.into() {
            257 => Some(Self::PathNameCreated { pathname: body_string(body) }),
            220 => Some(Self::ServiceReady { msg: body_string(body) }),
            _ => None
        }
    }
}

impl std::fmt::Display for FtpSuccessRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FtpSuccessRes::PathNameCreated { pathname } => write!(f, "257 PathNameCreated: {pathname}"),
            FtpSuccessRes::ServiceReady { msg } => write!(f, "220 ServiceReady: {msg}"),
        }
    }
}