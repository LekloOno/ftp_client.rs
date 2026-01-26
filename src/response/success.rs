use crate::{FtpResponseCode, response::FtpResponse};

pub enum FtpSuccessRes {
    PathNameCreated {pathname: String}
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
            _ => None
        }
    }
}

fn body_string(body: &[u8]) -> String {
    std::str::from_utf8(body).unwrap().to_string()
}

impl std::fmt::Display for FtpSuccessRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FtpSuccessRes::PathNameCreated { pathname } => write!(f, "{pathname}"),
        }
    }
}