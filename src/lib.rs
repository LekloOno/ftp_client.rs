pub mod stream;
pub mod response;
pub mod encode;
pub mod error;
pub mod basic;
pub mod command;

pub use crate::{
    response::FtpSuccessRes,
    stream::FtpStream,
    encode::FtpEncoder,
};

use std::io::{self, BufRead};


pub type Result<T> = std::result::Result<T, error::Error>;

mod private {
    /// Describes the base abstraction for an [`super::FtpClient`] internal behaviors.
    /// The methods implemented by this trait are meant to be used indirectly by
    /// the [`super::FtpClient`], not by the user. You shouldn't call any of these methods.
    /// 
    /// # Composition
    /// The trait enables the composition of a concrete [`super::FtpClient`].
    /// More precisely, an [`FtpClientInternal`] is defined by composing it with
    /// - any [`super::FtpStream`] implementation
    /// - any [`super::FtpEncoder`] implementation
    /// 
    /// ## Encoding
    /// FTP is encoding-agnostic, it does not specify the exact encoding format of its telnet strings.
    /// Thus, servers can use any, from ASCII, to ISO-8859-1, to EBCDIC, or UTF-8.
    /// 
    /// Such encoding is decouplated from the client's behavior through its [`FtpClientInternal::encoder`] component.
    /// It can be changed dynamically.
    /// 
    /// This would for example allow to send the `FEAT` command to retrieve the server's encoding 
    /// then change the client's encoder accordingly.
    /// 
    /// A simple standard UTF-8 encoder implementation is provided as [`super::basic::encoder::Utf8FtpEncoder`].
    /// 
    /// ## Stream
    /// FTP is mostly standardized to TCP, but the exact implementation could differ, plus old infrastructure 
    /// might still operate on top of Network Control Protocol instead, as FTP was originally designed for it.
    /// 
    /// Thus, an [`super::FtpClient`] can freely define the exact implementation to use.
    /// 
    /// > This might not require to be dynamically interchangeable, and we could argue this could be generically-typed 
    /// instead of dynamically dispatched. To argue.
    pub trait FtpClientInternal {
        /// Retrieves the current immutable FTP stream of the client.
        /// This assumes the client has only one stream, this might be improved later
        /// to enable asynchronous communications.
        /// 
        /// For most operations, a mutable reference is required.
        /// This could eventually be used to peek in the stream. 
        /// 
        /// See [`super::FtpStream`] documentation for more.
        ///
        /// # Returns
        /// An immutable reference to the current FTP stream of the client.
        fn stream(&self) -> &dyn super::FtpStream;
        /// Retrieves the current immutable FTP stream of the client.
        /// This assumes the client has only one stream, this might be improved later
        /// to enable asynchronous communications.
        /// 
        /// See [`super::FtpStream`] documentation for more.
        ///
        /// # Returns
        /// A mutable reference to the current FTP stream of the client.
        fn stream_mut(&mut self) -> &mut dyn super::FtpStream;
        /// Retrieves the current encoder used by the client.
        ///
        /// FTP is agnostic on the exact encoding method used for its telnet strings.
        /// This allows to 
        /// # Arguments
        /// * param1 - description
        /// * param2 - description
        ///
        /// # Returns
        /// Return description
        fn encoder(&self) -> &dyn super::FtpEncoder;
        /// Builds a new concrete [`FtpClientInternal`] over the provided `hostname` and optional `port`.
        ///
        /// # Arguments
        /// * hostname - The hostname of the FTP host, which can be a plain text IP.
        /// * ctrl_port - The port to use to set the control stream.
        ///
        /// # Returns
        /// A concrete instance of the implementing [`FtpClientInternal`] with a control port connected to the provided host.
        /// 
        /// # Errors
        /// To be specified by the implementor.
        /// See [`super::error::Error`].
        fn new(hostname: &str, ctrl_port: &str) -> super::Result<Self> where Self: Sized;
    } 
}

pub use private::FtpClientInternal as FtpClientInternal;

/// Describes the exposed interface of an FTP client.
/// 
/// This trait is fully pre-implemented upon the
/// underlying [`FtpClientInternal`] trait.
/// A blanket implementation already implements [`FtpClient`]
/// on any `T` implementing [`FtpClientInternal`].
/// 
/// The [`FtpClient`] is meant to abstract away any communication with the client in 
/// an explicit rust-idiomatic API.
/// 
/// It should ideally be used in pair with [`command::FtpCommand`].
/// Typically, for full explicitness, you wouldn't directly call [`FtpClient::send_command`] 
/// but rather build a command, and send it over the client.
pub trait FtpClient: FtpClientInternal {
    /// Builds a new concrete [`FtpClient`] over the provided `hostname` and optional `port`.
    ///
    /// # Arguments
    /// * hostname - The hostname of the FTP host, which can be a plain-text IP.
    /// * ctrl_port - Optional specific port of the control stream.
    ///     If `None`, uses the default port `21`.
    ///
    /// # Returns
    /// A concrete instance of the implementing [`FtpClient`] with a control port connected to the provided host.
    /// 
    /// # Errors
    /// Specific to the implementation.
    /// Would typically be things as a [`FtpStream`] [`stream::error::Error`] encapsulation.
    /// See [`error::Error`].
    fn new(hostname: &str, ctrl_port: Option<&str>) -> Result<Self> where Self: Sized {
        let eport = match ctrl_port {
            Some(p) => p,
            None => "21",
        };

        <Self as FtpClientInternal>::new(hostname, eport)
    }

    fn send_command(&mut self, command: &str) -> Result<()> {
        let encoded = self
            .encoder()
            .encode(&command)?;

        let crlf = self
            .encoder()
            .crlf();
        
        let ctrl_write = self
            .stream_mut()
            .control_write();
            
        ctrl_write
            .write_all(encoded)
            .and_then(|_| ctrl_write
                .write_all(crlf))
            .map_err(|e| error::Error::ControlStreamError(e))
    }

    fn send_data(&mut self, data: &[u8]) -> Result<()> {
        self.stream_mut()
            .data_write()
            .ok_or(error::Error::WriteUnitializedDataStream)?
            .write_all(data)
            .map_err(|e| error::Error::DataStreamError(e))
    }

    fn read_response(&mut self) -> Result<FtpSuccessRes> {
        let code = read_response_code(self)?;

        let hyphen = self.encoder().hyphen();
        let crlf = self.encoder().crlf();

        let mut buf_reader = io::BufReader::new(
            self.stream_mut().control_read()
        );
        let content = buf_reader.fill_buf()?;

        if content.starts_with(&hyphen) {
            todo!("Multiline responses not handled yet.");
        }

        let finder = memchr::memmem::Finder::new(crlf);
        
        let n_read = finder.find(content).unwrap(); // LAZY UNWRAP
        let body = &content[..n_read + crlf.len()];
        let response = response::response_from(code, body);

        buf_reader.consume(n_read + crlf.len());

        response.map_err(error::Error::from)
    }
}

fn read_response_code<T: FtpClient + ?Sized>(client: &mut T) -> Result<u8> {
    let code_len = client.encoder().response_code_len();
    let mut code_bytes = vec![0u8; code_len];
        
    client
        .stream_mut()
        .control_read()
        .read_exact(&mut code_bytes)?;
    
    client
        .encoder()
        .decode_response_code(&code_bytes)
        .map_err(|e| error::Error::EncoderError(e))
}

impl<T: FtpClientInternal> FtpClient for T {}