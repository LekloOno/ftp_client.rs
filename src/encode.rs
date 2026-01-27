pub mod error;

pub use error::Error;

use crate::response::FtpResponseCode;

pub type Result<T> = std::result::Result<T, Error>;

/// Describe the required methods to encode and decode data for
/// an FTP client-server connection.
/// 
/// Such encoding format could typically be retrieved with the `FEAT` command.
/// 
/// # Notes
/// The client is implemented in rust, and thus manipulates bytes as UTF-8 string when necessary. <br/>
/// 
/// ## Decoding
/// FTP does not require hosts to use any specific encoding format, thus, we want to abstract
/// this responsability away to be interchangeable.
/// 
/// Decoding is handled by -
/// - [`FtpEncoder::decode`]
/// - [`FtpEncoder::hyphen`]
/// - [`FtpEncoder::response_code_len`]
/// - [`FtpEncoder::decode_response_code`]
/// 
/// ## Encoding
/// Also, we of course need to encode sent commands to the host's format.
/// 
/// Encoding is handled by -
/// - [`FtpEncoder::encode`]
/// - [`FtpEncoder::crlf`]
pub trait FtpEncoder {
    /// Encodes the provided UTF-8 string to this encoder format as raw bytes.
    /// 
    /// # Lifetimes
    /// The raw bytes slice is not owned, it is referecing the provided string. 
    fn encode<'a>(&self, string: &'a str) -> Result<&'a [u8]>;
    /// Decodes raw bytes from this encoder format to UTF-8 string.
    /// 
    /// # Lifetimes
    /// The string is not owned, it is referencing the provided bytes.
    fn decode<'a>(&self, bytes: &'a [u8]) -> Result<&'a str>;

    /// Retrieves this encoding's definition of Carriage Return + Line Feed sequence (rust "\r\n")
    /// 
    /// # Notes
    /// It is notably used to encode commands, as they should end with such CRLF.<br/>
    /// This would usually be 0x0D and 0x0A.
    fn crlf(&self) -> &'static [u8];
    /// Retrieves this encoding's definition of a hyphen sequence (rust "-")
    /// 
    /// # Notes
    /// It is notably used to decode multi-line responses.
    /// This would usually be 0x2D.
    fn hyphen(&self) -> &'static [u8];
    
    /// Retrieves the number of bytes a response code takes in this encoder's format.
    /// 
    /// # Notes
    /// Essentially, this means three times the sizes of an numeric character,
    /// but it is not assumed.
    fn response_code_len(&self) -> usize;
    /// Tries to decode the provided bytes into an explicit [`FtpResponseCode`].
    fn decode_response_code(&self, bytes: &[u8]) -> Result<FtpResponseCode>;
}