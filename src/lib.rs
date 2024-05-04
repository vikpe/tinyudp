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

pub fn connect(address: &str) -> Result<UdpSocket> {
    let from_address = (Ipv4Addr::UNSPECIFIED, 0);
    let socket = UdpSocket::bind(from_address).map_err(|e| e!("tinyudp::connect: {}", e))?;
    socket.connect(address)?;
    Ok(socket)
}

pub fn send(address: &str, message: &[u8]) -> Result<UdpSocket> {
    let socket = connect(address)?;
    socket
        .send(message)
        .map_err(|e| e!("tinyudp::send: {}", e))?;
    Ok(socket)
}

pub fn read(socket: &UdpSocket, options: &ReadOptions) -> Result<Vec<u8>> {
    socket.set_read_timeout(options.timeout)?;

    let mut buffer: Vec<u8> = vec![0; options.buffer_size];
    let bytes_read = socket
        .recv(&mut buffer)
        .map_err(|e| e!("tinyudp::read: {}", e))?;

    let response = &buffer[..bytes_read];

    Ok(Vec::from(response))
}

pub fn send_and_read(address: &str, message: &[u8], read_options: &ReadOptions) -> Result<Vec<u8>> {
    let socket = send(address, message)?;
    read(&socket, read_options)
}
