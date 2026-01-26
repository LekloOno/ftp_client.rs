use crate::FtpClient;

use crate::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait FtpCommand: ToString {
    fn send(&self, client: &mut dyn FtpClient) -> Result<()> {
        client
            .send_command(&self.to_string())
    }
}