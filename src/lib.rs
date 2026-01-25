pub mod stream;
pub mod response;
pub mod encode;
pub mod error;

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
        fn stream_mut(&self) -> &mut dyn super::FtpStream;
        fn encoder(&self) -> &dyn super::encode::FtpEncoder;
    } 
}

pub use private::FtpClientInternal as FtpClientInternal;


pub trait FtpClient: private::FtpClientInternal {
    fn new(hostname: String) -> io::Result<Self> where Self: Sized;

    fn send_command(&mut self, command: &str) -> Result<()> {
        let encoded = self
            .encoder()
            .encode(&command)?;
        
        let ctrl_write = self
            .stream_mut()
            .control_write();
            
        ctrl_write
            .write_all(encoded)
            .and_then(|_| ctrl_write
                .write_all(self.encoder().crlf()))
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
        let ctrl_read = self
            .stream_mut()
            .control_read();

        let code = self.encoder().read_resp_code(ctrl_read)?;
        
        
        let mut buf_reader = io::BufReader::new(ctrl_read);
        let content = buf_reader.fill_buf()?;

        let hyphen = self.encoder().hyphen();
        if content.starts_with(&hyphen) {
            todo!("Multiline responses not handled yet.");
        }

        let crlf = self.encoder().crlf();
        let finder = memchr::memmem::Finder::new(crlf);
        
        let n_read = finder.find(content).unwrap(); // LAZY UNWRAP
        let body = &content[..n_read + crlf.len()];
        let response = FtpResponse::new(code, body);

        buf_reader.consume(n_read + crlf.len());

        Ok(response)
    }
}