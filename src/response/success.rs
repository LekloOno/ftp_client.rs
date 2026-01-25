pub enum FtpSuccessRes {
    PathNameCreated {pathname: String}
}

impl From<FtpSuccessRes> for super::FtpResponse {
    fn from(v: FtpSuccessRes) -> Self {
        Self::Ok(v)
    }
}

impl std::fmt::Display for FtpSuccessRes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FtpSuccessRes::PathNameCreated { pathname } => write!(f, "{pathname}"),
        }
    }
}