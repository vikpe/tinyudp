#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to bind socket: {0}")]
    BindFailed(#[source] std::io::Error),

    #[error("failed to send message: {0}")]
    SendFailed(#[source] std::io::Error),

    #[error("failed to receive message: {0}")]
    ReceiveFailed(#[source] std::io::Error),

    #[error("timeout reached while waiting for response")]
    TimeoutReached,
}
