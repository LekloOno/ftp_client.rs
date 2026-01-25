pub mod success;
pub mod error;

pub use success::FtpSuccessRes;
pub use error::{
    Error,
    error::FtpErrorRes,
    failure::FtpFailureRes,
};

pub type FtpResponse = std::result::Result<FtpSuccessRes, Error>;

pub fn response_from(code: u8, body: &[u8]) -> FtpResponse {
    if code & 200 == 200 {
        return Ok(FtpSuccessRes::PathNameCreated { pathname: std::str::from_utf8(body).unwrap().to_string() });
    }

    Err(Error::Error(FtpErrorRes::Default { res: std::str::from_utf8(body).unwrap().to_string() }))
}