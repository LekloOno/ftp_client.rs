use thiserror::Error;

pub mod error;
pub mod failure;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failure: {0}")]
    Failure(#[from] failure::FtpFailureRes),
    #[error("Error: {0}")]
    Error(#[from] error::FtpErrorRes),
}