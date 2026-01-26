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
    #[response_code(227)]
    EnteringPassiveMode {msg: String},
}

/// Should later take encoder into account.
pub fn parse_pasv_reply(msg: &str) -> Option<(String, u16)> {
    let start = msg.find('(')? + 1;
    let end = msg.find(')')?;
    let inside = &msg[start..end];

    let parts: Vec<u16> = inside
        .split(',')
        .filter_map(|p| p.trim().parse::<u16>().ok())
        .collect();

    if parts.len() != 6 {
        return None;
    }

    let ip = format!("{}.{}.{}.{}",
        parts[0], parts[1], parts[2], parts[3]
    );

    let port = parts[4] * 256 + parts[5];

    Some((ip, port))
}