use crate::{FtpClient, FtpClientInternal, basic::{encoder::Utf8FtpEncoder, stream::TcpFtpStream}, stream::FtpStream};

pub struct TcpFtpClient {
    stream: TcpFtpStream,
    encoder: Utf8FtpEncoder,
}

impl FtpClientInternal for TcpFtpClient {
    fn stream(&self) -> &dyn crate::FtpStream {
        &self.stream
    }

    fn stream_mut(&mut self) -> &mut dyn crate::FtpStream {
        &mut self.stream
    }

    fn encoder(&self) -> &dyn crate::encode::FtpEncoder {
        &self.encoder
    }

    fn new(hostname: &str, ctrl_port: &str) -> crate::Result<Self> where Self: Sized {
        Ok(Self {
            stream: TcpFtpStream::new(hostname, ctrl_port)?,
            encoder: Utf8FtpEncoder{},
        })
    }
}

impl FtpClient for TcpFtpClient {}