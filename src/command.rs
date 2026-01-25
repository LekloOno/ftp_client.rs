use crate::stream::FtpStream;

pub mod error;

pub type Result<T> = std::result::Result<T, error::Error>;

pub trait FtpCommand {
    fn send(&self, stream: &mut dyn FtpStream) -> Result<()> {
        stream
            .control_write()
            .write_all(self.as_bytes())
            .map_err(error::Error::from)
    }

    fn as_bytes(&self) -> &[u8];
}