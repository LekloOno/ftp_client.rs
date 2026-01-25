use crate::FtpClient;

use crate::Error;

pub type Result<T> = std::result::Result<T, Error>;

pub trait FtpCommand {
    fn send(&self, client: &mut dyn FtpClient) -> Result<()> {
        client
            .send_command(self.as_str())
    }

    fn as_str(&self) -> &str;
}