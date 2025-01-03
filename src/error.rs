use thiserror::Error;

#[derive(Error, Debug)]
pub enum KvsError {
    RmKeyError(String),
}

pub type Result<T> = std::result::Result<T, KvsError>;
