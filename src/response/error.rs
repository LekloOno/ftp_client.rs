use thiserror::Error;

use crate::{FtpResponseCode, response::FtpResponse};

pub mod error;
pub mod failure;

pub use error::FtpErrorRes;
pub use failure::FtpFailureRes;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failure: {0}")]
    Failure(#[from] FtpFailureRes),
    #[error("Error: {0}")]
    Error(#[from] FtpErrorRes),
    #[error("Response code unknown: {code}")]
    UnknownCode{ code: FtpResponseCode },
}

impl FtpResponse for Error {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code.outcome() {
            1 | 3 => FtpErrorRes::from_bytes(code, body).map(Error::from),
            4 | 5 => FtpFailureRes::from_bytes(code, body).map(Error::from),
            _ => None
        }
    }
}