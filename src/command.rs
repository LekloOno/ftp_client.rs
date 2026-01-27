use crate::FtpClient;

use crate::Error;

pub type Result<T> = std::result::Result<T, Error>;

/// Holds the behavior of [`FtpCommand`].  
/// 
/// It is somehow of a blanket implementation, as it is automatically derived from the [`ToString`].
pub trait FtpCommand: ToString {
    /// Sends the command over the provided [`FtpClient`].
    fn send(&self, client: &mut dyn FtpClient) -> Result<()> {
        client.send_command(&self.to_string())
    }

    /// Sends the command and waits for a response in the control stream of the provided [`FtpClient`].
    fn send_and_res(&self, client: &mut dyn FtpClient) -> crate::Result<crate::FtpSuccessRes> {
        client.send_command(&self.to_string())?;
        client.read_response()
    }
}