use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occured while writing on the control stream: {0}")]
    WriteError(#[from] std::io::Error),
}