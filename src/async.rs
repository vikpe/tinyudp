//! Async operations
use crate::{Error, ReadOptions};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::net::{ToSocketAddrs, UdpSocket};

/// Send a message to the specified address.
pub async fn send(target: impl ToSocketAddrs, message: &[u8]) -> Result<(), Error> {
    bind()
        .await?
        .send_to(message, target)
        .await
        .map_err(Error::SendFailed)?;
    Ok(())
}

/// Send a message to the specified address and read the response.
pub async fn send_and_receive(
    target: impl ToSocketAddrs,
    message: &[u8],
    options: ReadOptions,
) -> Result<Vec<u8>, Error> {
    let socket = bind().await?;

    // send
    socket
        .send_to(message, target)
        .await
        .map_err(Error::SendFailed)?;

    // receive
    let mut buffer = vec![0; options.buffer_size()];
    let (bytes_read, _) = tokio::select! {
        _ = tokio::time::sleep(options.timeout()) => Err(Error::TimeoutReached),
        res = socket.recv_from(&mut buffer) => res.map_err(Error::ReceiveFailed),
    }?;
    let response = buffer[..bytes_read].to_vec();
    Ok(response)
}

async fn bind() -> Result<UdpSocket, Error> {
    let address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    UdpSocket::bind(address).await.map_err(Error::BindFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReadOptions;
    use anyhow::Result;
    use std::time::Duration;

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_send() -> Result<()> {
        let res = send("quake.se:28501", b"\xff\xff\xff\xffstatus").await;
        assert!(res.is_ok());
        Ok(())
    }

    #[tokio::test]
    #[cfg_attr(feature = "ci", ignore)]
    async fn test_send_and_receive() -> Result<()> {
        {
            // timeout reached
            let response = send_and_receive(
                "quake.se:28501",
                b"\xff\xff\xff\xffstatus",
                ReadOptions::new(Duration::from_secs_f32(0.001), 32 * 1024),
            )
            .await;
            assert_eq!(
                response.unwrap_err().to_string(),
                "timeout reached while waiting for response"
            );
        }
        {
            // ok
            let response = send_and_receive(
                "quake.se:28501",
                b"\xff\xff\xff\xffstatus",
                ReadOptions::new(Duration::from_secs_f32(0.2), 32 * 1024),
            )
            .await?;
            assert!(String::from_utf8_lossy(&response).contains("QUAKE.SE KTX"));
        }
        Ok(())
    }
}
