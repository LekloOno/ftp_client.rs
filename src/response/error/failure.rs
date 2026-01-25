use thiserror::Error;

#[derive(Error, Debug)]
pub enum FtpFailureRes {
    #[error("Some failure : {res}")]
    Default {res: String}
}

impl From<FtpFailureRes> for crate::response::FtpResponse {
    fn from(v: FtpFailureRes) -> Self {
        Self::Err(v.into())
    }
}