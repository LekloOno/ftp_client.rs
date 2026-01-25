pub mod error;

pub use error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub trait FtpStream {
    fn new(hostname: &str, ctrl_port: &str) -> Result<Self> where Self: Sized;
    
    fn data_read(&mut self) -> Option<&mut dyn std::io::Read>;
    fn data_write(&mut self) -> Option<&mut dyn std::io::Write>;
    
    fn control_read(&mut self) -> &mut dyn std::io::Read;
    fn control_write(&mut self) -> &mut dyn std::io::Write;

    fn open_data(&mut self, hostname: &str, port: &str) -> Result<()>;
}