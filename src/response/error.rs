use thiserror::Error;

use crate::{FtpResponseCode, response::FtpResponse};

pub mod positive;
pub mod negative;

pub use negative::{
    permanent::FtpPermanentError,
    transient::FtpTransientError,
};

pub use positive::{
    preliminary::FtpPreliminaryReply,
    intermediate::FtpIntermediateReply,
};

#[derive(Error, Debug)]
pub enum Error {
    #[error("Positive Preliminary reply :\n{0}")]
    PreliminaryReply(#[from] FtpPreliminaryReply),
    #[error("Positive Intermediate reply :\n{0}")]
    IntermediateReply(#[from] FtpIntermediateReply),
    #[error("Permanent Negative Completion reply :\n{0}")]
    PermanentError(#[from] FtpPermanentError),
    #[error("Transient Negative Completion reply :\n{0}")]
    TransientError(#[from] FtpTransientError),
    #[error("Response code unknown: {code}")]
    UnknownCode{ code: FtpResponseCode },
}

impl FtpResponse for Error {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        match code.outcome() {
            1 => FtpPreliminaryReply::from_bytes(code, body).map(Error::from),
            3 => FtpIntermediateReply::from_bytes(code, body).map(Error::from),
            4 => FtpTransientError::from_bytes(code, body).map(Error::from),
            5 => FtpPermanentError::from_bytes(code, body).map(Error::from),
            _ => None
        }
    }
}