pub mod code;
pub mod success;
pub mod error;

pub use code::FtpResponseCode;
pub use success::FtpSuccessRes;
pub use error::{
    Error,
    error::FtpErrorRes,
    failure::FtpFailureRes,
};

/// # Implementation Details
/// The [`FtpResult`] type alias is willingly not called `Result` as other result aliases
/// in this crate, as this reslut has a slightly more intricated meaning.
/// 
/// It translates the semantic of FTP responses into a rust result semantic.
/// That is, receiving a response could be perceived a normal, non-faulty behavior.
/// We could thus simply always return the Ftp Response as an `Ok()` value.
/// 
/// But instead, we encapsulate FTP success response codes as `Ok`,
/// and error/failure as an `Err`, for more rust-idiomacy.
/// 
/// # Errors
/// The Error type can either represent a conventionnal valid Ftp Error
/// - [`Error::Error`] - 1yz or 3yz codes: Error or incomplete reply.
/// - [`Error::Failure`] - 4yz or 5yz codes: Failure reply.
/// Or a parsing error
/// - [`Error::UnknownCode`] - The xyz FTP response code is not supported.
pub type FtpResult = std::result::Result<FtpSuccessRes, Error>;

/// A helper trait to build FTP responses.
pub trait FtpResponse {
    /// Builds an FTP response instance the xyz response code.
    /// 
    /// # Implementation details
    /// For success responses, the first code digit (`2`) is filtered by the FtpResponse manager (currently, [`response_from`]).
    /// It's thus only necessary to match on the two last `2yz` digits, composing the kind and kind-details of the response. 
    /// 
    /// # Arguments
    /// * code - The response code, of type xyz.
    /// * body - The response body, might be empty. 
    ///
    /// # Returns
    /// None if the code is associated to no existing response under the target response category.
    fn from_bytes(code: &FtpResponseCode, body: &[u8]) -> Option<Self> where Self: Sized;
}

pub fn response_from(code: &FtpResponseCode, body: &[u8]) -> FtpResult {
    if code.outcome() == 2 {
        return FtpSuccessRes::from_bytes(code, body)
            .ok_or(Error::UnknownCode { code: code.clone() });
    }

    Err(Error::from_bytes(code, body)
        .unwrap_or(Error::UnknownCode { code: code.clone() }))
}