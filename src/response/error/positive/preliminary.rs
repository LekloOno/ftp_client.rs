use thiserror::Error;

use crate::{FtpResponseCode, response::FtpResponse};

#[derive(Debug, Error)]
pub enum FtpPreliminaryReply {

}

impl From<FtpPreliminaryReply> for crate::response::FtpResult {
    fn from(v: FtpPreliminaryReply) -> Self {
        Self::Err(v.into())
    }
}

impl FtpResponse for FtpPreliminaryReply {
    #[allow(unused)]
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized {
        None
    }
}