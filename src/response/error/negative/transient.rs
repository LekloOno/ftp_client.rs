use thiserror::Error;

use crate::response::{
    FtpResponse,
    FtpResult,
    FtpResponseCode,
    body_string
};

use ftp_response_derive::FtpResponseEnum;
use ftp_result_derive::ftp_result;

#[derive(Debug, Error, FtpResponseEnum)]
#[ftp_result(true)]
pub enum FtpTransientError {
    #[response_code(999)]
    UNDEFINED {msg: String},
}