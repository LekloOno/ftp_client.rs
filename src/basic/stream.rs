use std::net::TcpStream;

use crate::stream::{FtpStream, Result};
use crate::stream::Error;

pub struct TcpFtpStream {
    control: TcpStream,
    data: Option<TcpStream>,
}

impl FtpStream for TcpFtpStream {
    fn new(hostname: &str, ctrl_port: &str) -> Result<Self> where Self: Sized {
        let control = TcpStream::connect(format!("{hostname}:{ctrl_port}"))
            .map_err(|e| Error::ControlConnectionError(Box::new(e)))?;

        Ok(Self {
            control,
            data: None
        })
    }

    fn data_read(&mut self) -> Option<&mut dyn std::io::Read> {
        self.data.as_mut().map(|s| s as &mut dyn std::io::Read)
    }

    fn data_write(&mut self) -> Option<&mut dyn std::io::Write> {
        self.data.as_mut().map(|s| s as &mut dyn std::io::Write)
    }

    fn control_read(&mut self) -> &mut dyn std::io::Read {
        &mut self.control
    }

    fn control_write(&mut self) -> &mut dyn std::io::Write {
        &mut self.control
    }
    
    fn open_data_read(&mut self, hostname: &str, port: &str) -> Result<()> {
        let data_stream = TcpStream::connect(format!("{hostname}:{port}"))
            .map_err(|e| Error::DataConnectionError(Box::new(e)))?;
        
        self.data = Some(data_stream);

        Ok(())
    }

    fn open_data_write(&mut self, hostname: &str, port: &str) -> Result<()> {
        self.open_data_read(hostname, port)
    }
}