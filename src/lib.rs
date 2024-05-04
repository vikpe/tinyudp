//! # tinyudp
//! A tiny abstraction for UDP.
use std::net::{Ipv4Addr, SocketAddrV4, ToSocketAddrs, UdpSocket};
use std::time::Duration;

use anyhow::{anyhow as e, Result};

pub struct ReadOptions {
    pub timeout: Option<Duration>,
    pub buffer_size: usize,
}

impl Default for ReadOptions {
    fn default() -> Self {
        ReadOptions {
            timeout: None,
            buffer_size: 32 * 1024, // 32 kb
        }
    }
}

/// A UDP client that reuses the same socket for all operations.
pub struct Client {
    socket: UdpSocket,
}

impl Client {
    /// Create a new UDP client bound to the specified address.
    pub fn new(bind_address: impl ToSocketAddrs) -> Result<Self> {
        let socket = UdpSocket::bind(bind_address).map_err(|e| e!("tinyudp::bind: {}", e))?;
        Ok(Self { socket })
    }

    /// Create a new UDP client bound to an unspecified address.
    pub fn new_unspecified() -> Result<Self> {
        Self::new(SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0))
    }

    /// Send a message to the specified address.
    fn send(&self, address: impl ToSocketAddrs, message: &[u8]) -> Result<usize> {
        self.socket
            .send_to(message, address)
            .map_err(|e| e!("tinyudp::send: {}", e))
    }

    /// Read a message from the specified address.
    fn read(&self, address: impl ToSocketAddrs, options: &ReadOptions) -> Result<Vec<u8>> {
        self.socket.connect(address)?;
        self.socket.set_read_timeout(options.timeout)?;
        let mut buffer: Vec<u8> = vec![0; options.buffer_size];
        let bytes_read = self
            .socket
            .recv(&mut buffer)
            .map_err(|e| e!("tinyudp::read: {}", e))?;

        let response = &buffer[..bytes_read];

        Ok(Vec::from(response))
    }

    /// Send a message to the specified address and read the response.
    fn send_and_read(
        &self,
        address: impl ToSocketAddrs,
        message: &[u8],
        read_options: &ReadOptions,
    ) -> Result<Vec<u8>> {
        self.send(&address, message)?;
        self.read(&address, read_options)
    }
}

/// Send a message to the specified address.
pub fn send(address: impl ToSocketAddrs, message: &[u8]) -> Result<usize> {
    Client::new_unspecified()?.send(address, message)
}

/// Read a message from the specified address.
pub fn read(address: impl ToSocketAddrs, options: &ReadOptions) -> Result<Vec<u8>> {
    Client::new_unspecified()?.read(address, options)
}

pub fn send_and_read(
    address: impl ToSocketAddrs,
    message: &[u8],
    read_options: &ReadOptions,
) -> Result<Vec<u8>> {
    Client::new_unspecified()?.send_and_read(address, message, read_options)
}
