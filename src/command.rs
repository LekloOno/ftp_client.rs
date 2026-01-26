use crate::FtpClient;

use crate::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait FtpCommand: ToString {
    fn send(&self, client: &mut dyn FtpClient) -> Result<()> {
        client.send_command(&self.to_string())
    }

    fn send_and_res(&self, client: &mut dyn FtpClient) -> crate::Result<crate::FtpSuccessRes> {
        client.send_command(&self.to_string())?;
        client.read_response()
    }
}