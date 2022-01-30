# dogecoin miner rust

This miner is intended to only mine testnet Dogecoin.

## Start

Download the latest release : https://github.com/rllola/dogecoin-miner-rust/releases

Notes: Only available for macOS and Linux.

Unzip the executable using tar.

For Linux:
```bash
tar -xvf dogecoin-miner-rust-linux-amd64.tar.gz
```

For macOS:
```bash
tar -xvf dogecoin-miner-rust-macos-amd64.tar.gz
```

Create a `miner.toml` file next to your executable with the info to connect to your Dogecoin RPC node and/or Litecoin node.

Example of `miner.toml` file:
```toml
mergemining = true
address = "nbMFaHF9pjNoohS4fD1jefKBgDnETK9uPu"

[dogecoin]
ip = "127.0.0.1"
port = 44555
user = "kek"
password = "kek"

[litecoin]
ip = "127.0.0.1"
port = 19332
user = "kek"
password = "kek"
```

Start the executable:
```bash
./miner-rust
```

## Dev

You will need Cargo already installed. See https://doc.rust-lang.org/cargo/getting-started/installation.html

To setup your dev environnement with your regtest nodes you will need `docker` and `docker-compose` installed.

```bash
$ make up
```

And stop kill it
```bash
$ make down
```

To run the miner.
```bash
$ cargo r src/main.rs
```

## Troubleshooting

### Openssl error

```
  run pkg_config fail: "`\"pkg-config\" \"--libs\" \"--cflags\" \"openssl\"` did not exit successfully: exit status: 1\n--- stderr\nPackage openssl was not found in the pkg-config search path.\nPerhaps you should add the directory containing `openssl.pc'\nto the PKG_CONFIG_PATH environment variable\nNo package 'openssl' found\n"
```

You need to install `libssl-dev`.
```
$ apt install libssl-dev
```