use thiserror::Error;

#[derive(Error, Debug)]
pub enum FtpErrorRes {
    #[error("Some error : {res}")]
    Default {res: String}
}

impl From<FtpErrorRes> for crate::response::FtpResponse {
    fn from(v: FtpErrorRes) -> Self {
        Self::Err(v.into())
    }
}