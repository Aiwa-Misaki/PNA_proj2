//! kvStore crate
//!
//! This crate provides a simple key-value store implementation.
#![deny(missing_docs)]
pub use kv::KvStore;
/// err module containing KvsError
pub mod error;
mod kv;
pub use error::Result;
