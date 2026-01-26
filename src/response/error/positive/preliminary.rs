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
pub enum FtpPreliminaryReply {
    #[response_code(150)]
    FileStatusOkay {msg: String},
}