use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encoding error")]
    EncodingError,
    #[error("Decoding error")]
    DecodingError,
}