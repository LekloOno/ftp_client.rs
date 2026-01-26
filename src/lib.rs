pub mod stream;
pub mod response;
pub mod encode;
pub mod error;
pub mod basic;
pub mod command;
pub mod log;

pub use crate::{
    response::FtpSuccessRes,
    response::FtpResponseCode,
    stream::FtpStream,
    encode::FtpEncoder,
    error::Error,
};

use std::io::{self, BufRead};

pub type Result<T> = std::result::Result<T, Error>;

mod private {
    /// Describes the base abstraction for an [`super::FtpClient`] internal behaviors.
    /// The methods implemented by this trait are meant to be used indirectly by
    /// the [`super::FtpClient`], not by the user. You shouldn't call any of these methods.
    /// 
    /// # Notes
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
        /// A helper to quickly setup a base pasv data connection. <br/>
        /// It should open the data read stream.
        fn open_pasv_data(&mut self) -> super::Result<()>;
        /// Retrieves the current immutable FTP stream of the client.
        /// 
        /// # Notes
        /// For most operations, a mutable reference is required.
        /// This could eventually be used to peek in the stream. 
        /// 
        /// See [`super::FtpStream`] documentation for more.
        /// 
        /// # Implementation details
        /// This assumes the client has only one stream, this might be improved later
        /// to enable asynchronous communications.
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
        /// # Notes
        /// FTP is agnostic on the exact encoding method used for its telnet strings.
        /// This allows to dynamically interchange the encoding format used to match the host's,
        /// typically in tandem with the `FEAT` command.
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
        /// See [`super::Error`].
        fn new(hostname: &str, ctrl_port: &str) -> super::Result<Self> where Self: Sized;
    } 
}

pub use private::FtpClientInternal as FtpClientInternal;

/// Describes the exposed interface of an FTP client.
/// 
/// It is meant to abstract away any communication with the client in 
/// an explicit rust-idiomatic API.
/// 
/// # Notes
/// This trait is fully pre-implemented upon the
/// underlying [`FtpClientInternal`] trait.
/// A blanket implementation already implements [`FtpClient`]
/// on any `T` implementing [`FtpClientInternal`].
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
    /// Would typically be things as a [`FtpStream`] [`stream::Error`] encapsulation.
    /// See [`Error`].
    fn new(hostname: &str, ctrl_port: Option<&str>) -> Result<Self> where Self: Sized {
        let eport = match ctrl_port {
            Some(p) => p,
            None => "21",
        };

        <Self as FtpClientInternal>::new(hostname, eport)
    }
    /// Sends the provided UTF-8 string command to the host over the control stream.
    ///  
    /// # Notes
    /// Only the pure UTF-8 command content is expected.
    /// The method handles proper encoding and formating.
    /// 
    /// ## Transcoding
    /// The method is responsible for transcoding the string to the proper format.
    /// 
    /// ## Formating
    /// The method ensures the transmitted command always ends with the proper CRLF.
    /// 
    /// # Arguments
    /// * command - an UTF-8 command to pass over the control stream.
    /// 
    /// # Errors
    /// Common errors accross implementations are -
    /// - [`Error::EncoderError`] if the encoder fails encoding the command, see [`encode::Error`].
    /// - [`Error::ControlStreamError`] if an error occured while writing to the control stream, see [`stream::Error`].
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
            .map_err(|e| Error::ControlStreamError(e))
    }

    /// Sends the command and waits for the server's response.
    /// See [`FtpClient::send_command`] and [`FtpClient::read_response`].
    fn send_command_and_listen(&mut self, command: &str) -> Result<FtpSuccessRes> {
        self.send_command(command)?;
        self.read_response()
    }
    /// Sends the content of the bytes buffer over the data stream.  
    ///
    /// # Arguments
    /// * data - a buffer of bytes.
    /// 
    /// # Errors
    /// Common errors accross implementations are -
    /// - [`Error::WriteUnitializedDataStream`] if there is no data write stream initialized.
    /// - [`Error::DataStreamError`] if an error occured while writing to the data stream, see [`stream::Error`].
    fn send_data(&mut self, data: &[u8]) -> Result<()> {
        self.stream_mut()
            .data_write()
            .ok_or(Error::WriteUnitializedDataStream)?
            .write_all(data)
            .map_err(|e| Error::DataStreamError(e))
    }
    /// Reads and retrieves a response from the control stream.
    /// 
    /// # Notes
    /// The method extract the underlying [`FtpSuccessRes`] if the response is successfull,
    /// or propagate the response as an [`Error::ResponseError`] if it is either an
    /// [`response::FtpErrorRes`] or an [`response::FtpFailureRes`]. 
    /// 
    /// # Panics
    /// Multiline responses are not handled yet.
    /// The method panics if it meets one.
    /// 
    /// The method can also panic if no CRLF is found in the control stream.
    /// This should be safely handled later, especially as the client might just read in the control too soon ?
    /// 
    /// # Returns
    /// The successful response contained next in the control stream.
    /// 
    /// # Errors
    /// Common errors accros implementations are -
    /// - [`Error::ResponseError`] if a response has been successfully read, but contained an error or failure code.
    ///     see [`response::FtpErrorRes`] and [`response::FtpFailureRes`].
    /// - [`Error::ControlStreamError`] if an error occured while reading the control stream, see [`stream::Error`].
    /// - [`Error::IoError`] if an error occured while buffer-reading the control stream, see [`io::Error`].
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
        let response = response::response_from(&code, body);

        buf_reader.consume(n_read + crlf.len());

        response.map_err(Error::from)
    }

    /// Retrieves the data read stream. <br/>
    /// This can typically be used to pipe the stream into a Write stream for fast data polling.
    /// 
    /// # Errors
    /// Common errors accros implementations are -
    /// - [`Error::ReadUnitializedDataStream`] if there is no data read stream initialized.
    fn data_read_stream(&mut self) -> Result<&mut dyn std::io::Read> {
        self.stream_mut()
            .data_read()
            .ok_or(Error::ReadUnitializedDataStream)
    }
    
    /// Reads the data stream into the provided buffer.
    ///
    /// # Arguments
    /// * buff - a bytes buffer to read data in.
    ///
    /// # Returns
    /// The number of bytes read.
    /// 
    /// # Errors
    /// Common errors accros implementations are -
    /// - [`Error::ReadUnitializedDataStream`] if there is no data read stream initialized.
    /// - [`Error::DataStreamError`] if an error occured while writing to the data stream, see [`stream::Error`].
    fn read_data(&mut self, buff: &mut [u8]) -> Result<usize> {
        self.stream_mut()
            .data_read()
            .ok_or(Error::ReadUnitializedDataStream)?
            .read(buff)
            .map_err(|e| Error::DataStreamError(e))
    }

    /// Reads the data stream into the provided `Vec<u8>` buffer until an `EOF` is met.
    ///
    /// # Arguments
    /// * buff - a dynamically sized bytes buffer.
    ///
    /// # Errors
    /// Common errors accros implementations are -
    /// - [`Error::ReadUnitializedDataStream`] if there is no data read stream initialized.
    /// - [`Error::DataStreamError`] if an error occured while writing to the data stream, see [`stream::Error`].
    fn read_data_to_end(&mut self, buff: &mut Vec<u8>) -> Result<()> {
        self.stream_mut()
            .data_read()
            .ok_or(Error::ReadUnitializedDataStream)?
            .read_to_end(buff)
            .map_err(|e| Error::DataStreamError(e))
            .map(|_| ())
    }

    /// Opens a data read stream.
    ///
    /// # Arguments
    /// * hostname  - The hostname of the FTP host, which can be a plain-text IP. 
    /// * port      - The port of the data read stream.
    ///
    /// # Notes
    /// Opening a data read stream as one is already opened is unspecified behavior. <br/>
    /// It might fail, close the existing one, or anything else. It is up to the stream implementation.
    /// 
    /// # Errors
    /// See [`stream::Error`].
    fn open_data_read(&mut self, hostname: &str, port: &str) -> Result<()> {
        self.stream_mut().open_data_read(hostname, port).map_err(Error::from)
    }

    /// Opens a data write stream.
    ///
    /// # Arguments
    /// * hostname  - The hostname of the FTP host, which can be a plain-text IP. 
    /// * port      - The port of the data write stream.
    ///
    /// # Notes
    /// Opening a data write stream as one is already opened is unspecified behavior. <br/>
    /// It might fail, close the existing one, or anything else. It is up to the stream implementation.
    /// 
    /// # Errors
    /// See [`stream::Error`].
    fn open_data_write(&mut self, hostname: &str, port: &str) -> Result<()> {
        self.stream_mut().open_data_write(hostname, port).map_err(Error::from)
    }

    /// A helper to quickly setup a base pasv data connection. <br/>
    fn open_pasv_data(&mut self) -> Result<()> {
        FtpClientInternal::open_pasv_data(self)
    }
}

fn read_response_code<T: FtpClient + ?Sized>(client: &mut T) -> Result<FtpResponseCode> {
    let code_len = client.encoder().response_code_len();
    let mut code_bytes = vec![0u8; code_len];
        
    client
        .stream_mut()
        .control_read()
        .read_exact(&mut code_bytes)
        .map_err(|e| Error::ControlStreamError(e))?;
    
    client
        .encoder()
        .decode_response_code(&code_bytes)
        .map_err(|e| Error::EncoderError(e))
}

impl<T: FtpClientInternal> FtpClient for T {}