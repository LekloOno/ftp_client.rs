use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("An error occured on a i/o stream: {0}")]
    IoError(#[from] std::io::Error), 
    #[error("Error occured on the control stream: {0}")]
    ControlStreamError(std::io::Error),
    #[error("Error occured on the data stream: {0}")]
    DataStreamError(std::io::Error),
    #[error("Trying to read on data stream, but it is not opened.")]
    ReadUnitializedDataStream,
    #[error("Trying to write on data stream, but it is not opened.")]
    WriteUnitializedDataStream,
    #[error("An error occured while encoding data: {0}")]
    EncoderError(#[from] crate::encode::Error),
    #[error("An error occured while opening a stream: {0}")]
    StreamError(#[from] crate::stream::Error),
    #[error("Response contains an error: {0}")]
    ResponseError(#[from] crate::response::Error)
}