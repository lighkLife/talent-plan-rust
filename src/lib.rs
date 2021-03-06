#![deny(missing_docs)]
//! A simple key-value storage.
pub use client::KvsClient;
pub use engines::{KvsEngine, KvStore, SledKvsEngine};
pub use err::{KvsError, Result};
pub use server::KvServer;

mod err;
mod protocol;
mod client;
mod server;
mod engines;
/// thread pool
pub mod thread_pool;

