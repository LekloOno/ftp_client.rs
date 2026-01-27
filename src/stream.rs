pub mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

/// FtpStream defines the abstract requirements for an FTP Stream.
/// 
/// # Notes
/// The goal of such abstraction is multiple.
/// - First, it allows to better match the protocol-side vision of FTP with no further assumptions.
/// - Secondly, although TCP has become the standard, it isn't necessarily. FTP used to run on NCP.
/// I suppose it is very unlikely servers still run on it, but it is at least fully agnostic of the
/// exact transport protocol. 
pub trait FtpStream {
    fn new(hostname: &str, ctrl_port: &str) -> Result<Self> where Self: Sized;
    
    /// Tries to retrieve the data read stream.
    /// 
    /// # Returns
    /// `None` if the stream is not initialized yet.
    fn data_read(&mut self) -> Option<&mut dyn std::io::Read>;
    /// Tries to retrieve the data write stream.
    /// 
    /// # Returns
    /// `None` if the stream is not initialized yet.
    fn data_write(&mut self) -> Option<&mut dyn std::io::Write>;
    
    /// Retrieves the control read stream.
    fn control_read(&mut self) -> &mut dyn std::io::Read;
    /// Retrieves the control write stream.
    fn control_write(&mut self) -> &mut dyn std::io::Write;

    /// Opens the data read stream on the specified hostname and port. <br/>
    /// Such hostname and port could typically be retrieved from PASV command response. 
    /// 
    /// # Notes
    /// On a simple TCP implementation, [`FtpStream::open_data_write`] is the same. <br/>
    /// Nothing is defined on the behavior this method should have besides that it should
    /// try to open the stream on the provided hostname:port. Typically it is not specified
    /// what should happen if one tries to open multiple data stream at once.
    fn open_data_read(&mut self, hostname: &str, port: &str) -> Result<()>;
    /// Opens the data write stream on the specified hostname and port. <br/>
    /// Such hostname and port could typically be retrieved from PASV command response.
    /// 
    /// # Notes
    /// On a simple TCP implementation, [`FtpStream::open_data_read`] is the same. <br/>
    /// Nothing is defined on the behavior this method should have besides that it should
    /// try to open the stream on the provided hostname:port. Typically it is not specified
    /// what should happen if one tries to open multiple data stream at once.
    fn open_data_write(&mut self, hostname: &str, port: &str) -> Result<()>;
}