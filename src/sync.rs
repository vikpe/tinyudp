//! Sync operations
use crate::{Error, ReadOptions};
use std::net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs, UdpSocket};

/// Send a message to the specified address.
pub fn send(target: impl ToSocketAddrs, message: &[u8]) -> Result<(), Error> {
    bind()?
        .send_to(message, target)
        .map_err(Error::SendFailed)?;
    Ok(())
}

/// Send a message to the specified address and read the response.
pub fn send_and_receive(
    target: impl ToSocketAddrs,
    message: &[u8],
    options: ReadOptions,
) -> Result<Vec<u8>, Error> {
    let socket = bind()?;

    // send
    socket.send_to(message, target).map_err(Error::SendFailed)?;
    socket
        .set_read_timeout(Some(options.timeout()))
        .map_err(|_| Error::TimeoutReached)?;

    // receive
    let mut buffer = vec![0; options.buffer_size()];
    let (bytes_read, _) = socket
        .recv_from(&mut buffer)
        .map_err(Error::ReceiveFailed)?;
    let response = buffer[..bytes_read].to_vec();
    Ok(response)
}

fn bind() -> Result<UdpSocket, Error> {
    let address = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    UdpSocket::bind(address).map_err(Error::BindFailed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ReadOptions;
    use anyhow::Result;
    use std::time::Duration;

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_send() {
        let res = send("quake.se:28501", b"\xff\xff\xff\xffstatus");
        assert!(res.is_ok());
    }

    #[test]
    #[cfg_attr(feature = "ci", ignore)]
    fn test_send_and_receive() -> Result<()> {
        let response = send_and_receive(
            "quake.se:28501",
            b"\xff\xff\xff\xffstatus",
            ReadOptions::new(Duration::from_secs_f32(0.2), 32 * 1024),
        )?;
        assert!(String::from_utf8_lossy(&response).contains("QUAKE.SE KTX"));
        Ok(())
    }
}
