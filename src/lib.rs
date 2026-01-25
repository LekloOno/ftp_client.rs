pub mod stream;
pub mod response;
pub mod encode;
pub mod error;

use crate::stream::FtpStream;


pub type Result<T> = std::result::Result<T, error::Error>;

// I don't know yet how to truly make the internal part private
// I leave it for structure
mod private {
    pub trait FtpClientInternal {
        fn stream(&self) -> &dyn super::FtpStream;
        fn stream_mut(&self) -> &mut dyn super::FtpStream;
        fn encoder(&self) -> &dyn super::encode::FtpEncoder;
    } 
}

pub use private::FtpClientInternal as FtpClientInternal;
