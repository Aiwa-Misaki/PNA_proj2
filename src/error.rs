use thiserror::Error;

#[derive(Error, Debug)]
/// err
pub enum KvsError {
    #[error("IO error: {0}")]
    /// error related to file IO
    IOError(#[from] std::io::Error),

    #[error("Key not found")]
    /// error related to remove key
    RmKeyError(String),

    #[error("serialization error: {0}")]
    /// error related to serialize & deserialize
    SerializeError(#[from] serde_json::Error),
}


/// alias for Result<T, KvsError>
pub type Result<T> = std::result::Result<T, KvsError>;
