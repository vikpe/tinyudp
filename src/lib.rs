//! # tinyudp
//! A tiny abstraction for UDP.

// Internals
mod error;
mod options;
mod sync;

#[cfg(feature = "tokio")]
mod r#async;

// Public interface
pub use error::Error;
pub use options::ReadOptions;
pub use sync::{send, send_and_receive};

#[cfg(feature = "tokio")]
pub use r#async::{send as send_async, send_and_receive as send_and_receive_async};
