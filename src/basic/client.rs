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
    username: String,
    password: String,
    stream: TcpFtpStream,
    encoder: Utf8FtpEncoder,
}

impl FtpClientInternal for TcpFtpClient {
    fn new(hostname: &str, ctrl_port: &str) -> crate::Result<Self> where Self: Sized {
        Ok(Self {
            hostname: hostname.to_string(),
            port: ctrl_port.to_string(),
            username: "anonymous".to_string(),
            password: "".to_string(),
            stream: TcpFtpStream::new(hostname, ctrl_port)?,
            encoder: Utf8FtpEncoder{},
        })
    }
    
    fn hostname(&self) -> &str { &self.hostname }
    fn port(&self) -> &str { &self.port }
    fn username(&self) -> &str { &self.username }
    fn password(&self) -> &str { &self.password }

    fn stream(&self) -> &dyn crate::FtpStream { &self.stream }
    fn stream_mut(&mut self) -> &mut dyn crate::FtpStream { &mut self.stream }
    fn encoder(&self) -> &dyn crate::encode::FtpEncoder { &self.encoder }
    
    fn connect(&mut self, hostname: &str, ctrl_port: &str) -> crate::Result<()> {
        self.stream = TcpFtpStream::new(hostname, ctrl_port)?;
        self.read_response()?;
        Ok(())
    }

    fn open_pasv_data_internal(&mut self) -> crate::Result<()> {
        let _ = &FtpBaseCommands::Pasv.send(self)?;
        let res = FtpClient::read_response(self)?;

        match res {
            crate::FtpSuccessRes::ServiceReady { msg: _ } => Ok(()),
            crate::FtpSuccessRes::EnteringPassiveMode { msg } => {
                let (ip, port) = crate::response::success::parse_pasv_reply(&msg).expect("msg");

                self.open_data_read(&ip.to_string(), &port.to_string())
                    .map_err(crate::Error::from)
            },
            _ => panic!("Unexpected response {res}"),
        }
    }
    
    fn authenticate_internal(&mut self) -> crate::Result<()> {
        let user_cmd = FtpBaseCommands::User { username: "anonymous".to_string() };
        is_valid_auth_res(user_cmd.send_and_res(self))?;

        let pswd_cmd = FtpBaseCommands::Pass { password: "pswd".to_string() };
        is_valid_auth_res(pswd_cmd.send_and_res(self))?;
        Ok(())
    }
}

fn is_valid_auth_res(res: crate::Result<crate::FtpSuccessRes>) -> crate::Result<()> {
    let Err(error) = res else {
        return Ok(());
    };

    let crate::Error::ResponseError(res) = &error else {
        return Err(error);
    };

    match res {
        crate::response::Error::PreliminaryReply(_ftp_preliminary_reply) => Ok(()),
        | crate::response::Error::IntermediateReply(_ftp_intermediate_reply) => Ok(()),
        _ => Err(error),
    }
}