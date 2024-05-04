//! # tinyudp
//! A tiny abstraction for UDP.
use std::net::{Ipv4Addr, UdpSocket};
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
            buffer_size: 8 * 1024, // 8 kb
        }
    }
}

pub struct Client {
    socket: UdpSocket,
}

impl Client {
    pub fn new(bind_address: (Ipv4Addr, u16)) -> Result<Self> {
        let socket = UdpSocket::bind(bind_address).map_err(|e| e!("tinyudp::bind: {}", e))?;
        Ok(Self { socket })
    }

    pub fn new_unspecified() -> Result<Self> {
        Self::new((Ipv4Addr::UNSPECIFIED, 0))
    }

    fn send(&self, address: &str, message: &[u8]) -> Result<usize> {
        self.socket
            .send_to(message, address)
            .map_err(|e| e!("tinyudp::send: {}", e))
    }

    fn read(&self, address: &str, options: &ReadOptions) -> Result<Vec<u8>> {
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

    fn send_and_read(
        &self,
        address: &str,
        message: &[u8],
        read_options: &ReadOptions,
    ) -> Result<Vec<u8>> {
        self.send(address, message)?;
        self.read(address, read_options)
    }
}

pub fn send(address: &str, message: &[u8]) -> Result<usize> {
    Client::new_unspecified()?.send(address, message)
}

pub fn read(address: &str, options: &ReadOptions) -> Result<Vec<u8>> {
    Client::new_unspecified()?.read(address, options)
}

pub fn send_and_read(address: &str, message: &[u8], read_options: &ReadOptions) -> Result<Vec<u8>> {
    Client::new_unspecified()?.send_and_read(address, message, read_options)
}
