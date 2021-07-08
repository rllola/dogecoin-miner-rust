# dogecoin miner rust

This miner is intended to only mine testnet Dogecoin.

## Start

You will need Cargo already installed. See https://doc.rust-lang.org/cargo/getting-started/installation.html

```
$ cargo build --release
```

You will find your executable `miner-rust` under `target/release/`.

Create a `.env` file next to your executable with the info to connect to your Dogecoin RPC testnet node.

Example of `.env` file:
```
127.0.0.1
44555
username
password
```