use crate::response::{
    FtpResponse,
    FtpResult,
    FtpResponseCode,
    body_string
};

use ftp_response_derive::FtpResponseEnum;
use ftp_result_derive::ftp_result;

#[derive(Debug, FtpResponseEnum)]
#[ftp_result(false)]
pub enum FtpSuccessRes {
    #[response_code(220)]
    ServiceReady { msg: String },
    #[response_code(230)]
    UserLoggedInProceed {msg: String},
    #[response_code(257)]
    PathNameCreated {msg: String},
}