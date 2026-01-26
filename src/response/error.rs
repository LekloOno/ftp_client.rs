use thiserror::Error;

use crate::{FtpResponseCode, response::FtpResponse};

pub mod error;
pub mod failure;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failure: {0}")]
    Failure(#[from] failure::FtpFailureRes),
    #[error("Error: {0}")]
    Error(#[from] error::FtpErrorRes),
    #[error("Response code unknown: {code}")]
    UnknownCode{ code: FtpResponseCode },
}

impl FtpResponse for Error {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        None
    }
}