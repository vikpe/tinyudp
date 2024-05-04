# tinyudp

> A tiny abstraction for UDP in Rust

## Overview

```rust
tinyudp::connect(address: &str) -> Result<UdpSocket>
tinyudp::send(address: &str, message: &[u8]) -> Result<()>
tinyudp::read(socket: &UdpSocket, options: &ReadOptions) -> Result<Vec<u8>>

pub struct ReadOptions {
    pub timeout: Option<Duration>,
    pub buffer_size: usize,
}
```

## Usage

### Send

```rust
tinyudp::send("quake.se", &b"hello")?;
```

### Read

```rust
let socket = tinyudp::connect("quake.se")?;
let options = tinyudp::ReadOptions{
    timeout: Some(Duration::from_secs(1)),
    buffer_size: 8 * 1024,
};
let response = tinyudp::read(&socket, &options)?;
```

### Send and read

```rust
let message = b"hello";
let options = tinyudp::ReadOptions{
    timeout: Some(Duration::from_secs(1)),
    buffer_size: 8 * 1024,
};

match tinyudp::send_and_read("quake.se", &message, &options) {
    Ok(response) => {
        println!("response: {:?}", response);
    },
    Err(e) => {
        println!("error: {:?}", e);
    },
};
```
