# tinyudp [![Test](https://github.com/vikpe/tinyudp/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/tinyudp/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/tinyudp)](https://crates.io/crates/tinyudp) [![docs.rs](https://img.shields.io/docsrs/tinyudp)](https://docs.rs/tinyudp/)

> A tiny abstraction for UDP in Rust

## Overview

```rust
(async) tinyudp::send(address: impl ToSocketAddrs, message: &[u8]) -> Result<(), TinyudpError>
(async) tinyudp::send_and_receive(address: impl ToSocketAddrs, message: &[u8], read_options: &ReadOptions) -> Result<Vec<u8>, TinyudpError>

struct ReadOptions {
    pub timeout: Duration,
    pub buffer_size: usize,
}
```

## Usage

### Send

```rust
tinyudp::send("quake.se:28000", &b"hello").await?;
```

### Send and receive

```rust
let options = tinyudp::ReadOptions{
    timeout: Some(Duration::from_secs(1)),
    buffer_size: 8 * 1024,
};

match tinyudp::send_and_receive("quake.se:28000", &b"hello", &options).await {
    Ok(response) => {
        println!("response: {:?}", response);
    },
    Err(e) => {
        println!("error: {:?}", e);
    },
};
```
