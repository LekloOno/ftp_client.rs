use crate::{FtpClient, FtpClientInternal, basic::{command::FtpBaseCommands, encoder::Utf8FtpEncoder, stream::TcpFtpStream}, command::FtpCommand, stream::FtpStream};

pub struct TcpFtpClient {
    stream: TcpFtpStream,
    encoder: Utf8FtpEncoder,
    hostname: String,
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
            stream: TcpFtpStream::new(hostname, ctrl_port)?,
            encoder: Utf8FtpEncoder{},
        })
    }
    
    fn open_pasv_data_internal(&mut self) -> crate::Result<()> {
        let _ = &FtpBaseCommands::Pasv.send(self)?;
        let res = FtpClient::read_response(self)?;

        let crate::FtpSuccessRes::EnteringPassiveMode {msg} = res else {
            panic!("Unexpected response");
        };

        let (ip, port) = crate::response::success::parse_pasv_reply(&msg).expect("msg");

        println!("{}:{}", ip, port);

        self.open_data_read(&self.hostname.to_string(), &port.to_string())
            .map_err(crate::Error::from)
    }
}