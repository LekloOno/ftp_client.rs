pub mod stream;
pub mod response;
pub mod encode;
pub mod error;
pub mod basic;
pub mod command;

use crate::{
    response::FtpResponse,
    stream::FtpStream
};

use std::io::{self, BufRead};


pub type Result<T> = std::result::Result<T, error::Error>;

// I don't know yet how to truly make the internal part private
// I leave it for structure
mod private {
    pub trait FtpClientInternal {
        fn stream(&self) -> &dyn super::FtpStream;
        fn stream_mut(&mut self) -> &mut dyn super::FtpStream;
        fn encoder(&self) -> &dyn super::encode::FtpEncoder;
    } 
}

pub use private::FtpClientInternal as FtpClientInternal;


pub trait FtpClient: private::FtpClientInternal {
    fn new(hostname: &str, port: Option<&str>) -> Result<Self> where Self: Sized;

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

    fn read_response(&mut self) -> Result<FtpResponse> {
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
        let response = FtpResponse::new(code, body);

        buf_reader.consume(n_read + crlf.len());

        Ok(response)
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