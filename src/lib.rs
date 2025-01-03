//! kvStore crate
//!
//! This crate provides a simple key-value store implementation.
#![deny(missing_docs)]
pub use kv::KvStore;
mod error;
mod kv;
pub use error::Result;

fn main() {
    let store;
    let res = KvStore::open("".path());
    match res {
        Ok(st) => store = st,
        Err(err) => return err,
    }
}
