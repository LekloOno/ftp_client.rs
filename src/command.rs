use crate::FtpClient;

pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait FtpCommand {
    fn send(&self, client: &mut dyn FtpClient) -> Result<()> {
        client
            .send_command(self.as_str())
            .map_err(error::Error::from)
    }

    fn as_str(&self) -> &str;
}