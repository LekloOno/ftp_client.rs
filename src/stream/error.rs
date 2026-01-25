use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Couldn't establish control connection")]
    ControlConnectionError(Box<dyn std::error::Error>),
    #[error("Couldn't establish data connection")]
    DataConnectionError(Box<dyn std::error::Error>),
}