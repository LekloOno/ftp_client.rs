use crate::{FtpClient, FtpClientInternal, basic::{command::FtpBaseCommands, encoder::Utf8FtpEncoder, stream::TcpFtpStream}, command::FtpCommand, stream::FtpStream};

/// Provides a standard TCP based FTP client.
/// 
/// # Notes
/// It is composed of
/// - a [`TcpFtpStream`]
/// - a [`Utf8FtpEncoder`]
pub struct TcpFtpClient {
    hostname: String,
    port: String,
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
            hostname: hostname.to_string(),
            port: ctrl_port.to_string(),
            stream: TcpFtpStream::new(hostname, ctrl_port)?,
            encoder: Utf8FtpEncoder{},
        })
    }
    
    fn open_pasv_data_internal(&mut self) -> crate::Result<()> {
        let _ = &FtpBaseCommands::Pasv.send(self)?;
        let res = FtpClient::read_response(self)?;

        let crate::FtpSuccessRes::EnteringPassiveMode {msg} = res else {
            panic!("Unexpected response {res}");
        };

        let (ip, port) = crate::response::success::parse_pasv_reply(&msg).expect("msg");

        self.open_data_read(&ip.to_string(), &port.to_string())
            .map_err(crate::Error::from)
    }
    
    fn connect(&mut self, hostname: &str, ctrl_port: &str) -> crate::Result<()> {
        self.stream = TcpFtpStream::new(hostname, ctrl_port)?;
        Ok(())
    }
    
    fn hostname(&self) -> &str { &self.hostname }
    fn port(&self) -> &str { &self.port }
}