# tinyudp [![Test](https://github.com/vikpe/tinyudp/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/tinyudp/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/tinyudp)](https://crates.io/crates/tinyudp) [![docs.rs](https://img.shields.io/docsrs/tinyudp)](https://docs.rs/tinyudp/)

> A tiny abstraction for UDP in Rust

## Overview

```rust
tinyudp::send(address: &str, message: &[u8]) -> Result<usize>
tinyudp::read(address: &str, options: &ReadOptions) -> Result<Vec<u8>>
tinyudp::send_and_read(address: &str, message: &[u8], read_options: &ReadOptions) -> Result<Vec<u8>>

struct ReadOptions {
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
