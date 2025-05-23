use std::time::Duration;

/// Options for reading responses with timeout and buffer size.
#[derive(Debug, Clone, Copy)]
pub struct ReadOptions {
    /// Size of the read buffer in bytes.
    buffer_size: usize,

    /// Duration to wait before timing out.
    timeout: Duration,
}

impl ReadOptions {
    pub fn new(timeout: Duration, buffer_size: usize) -> Self {
        Self {
            timeout,
            buffer_size,
        }
    }

    pub fn buffer_size(&self) -> usize {
        self.buffer_size
    }

    pub fn timeout(&self) -> Duration {
        self.timeout
    }
}
