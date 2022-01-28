# dogecoin miner rust

This miner is intended to only mine testnet Dogecoin.

## Start

You will need Cargo already installed. See https://doc.rust-lang.org/cargo/getting-started/installation.html

```
$ cargo build --release
```

You will find your executable `miner-rust` under `target/release/`.

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

## Dev

To setup your dev environnement with your regtest nodes you will need `docker` and `docker-compose` installed.

```bash
$ make up
```

And stop kill it
```bash
$ make down
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