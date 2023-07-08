use serde::Deserialize;
use std::fs::File;
use std::io::prelude::*;
use toml;

#[derive(Deserialize)]
pub struct ConfigRPC {
    pub ip: String,
    pub port: u16,
    pub user: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub mergemining: bool,
    pub address: String,
    pub dogecoin: ConfigRPC,
    pub litecoin: Option<ConfigRPC>,
}

pub fn read_config() -> Config {
    let mut file = File::open("miner.toml").expect("miner.toml file required");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let config: Config = toml::from_str(&contents).unwrap();

    return config;
}
