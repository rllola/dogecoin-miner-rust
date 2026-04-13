use clap::{arg, value_parser, Command};

mod configs;
mod miner;
mod rpc;
mod utils;

fn main() {
    let matches = Command::new("Miner")
        .about("Dogecoin miner to mine testnet. It also support merge mining.")
        .arg(arg!(-c --config <FILE> "Specify configuation file.")
                .value_parser(value_parser!(String))
                .default_value("./miner.toml"),
        )
        .get_matches();

    let config_path = matches.get_one::<String>("config").expect("config file path to be specified or default to './miner.toml'");

    let config = configs::read_config(config_path);

    // Verifying address is correct
    // TODO: verify length is 20 bytes (correct size for a pubkey hash)
    let pubkeyhash =
        utils::address_to_pubkeyhash(&config.address).expect("`address` is not a correct address");

    if config.mergemining {
        println!("START MERGE MINE!");
        loop {
            miner::merge_mine(&config, &pubkeyhash);
        }
    } else {
        loop {
            miner::mine(&config, &pubkeyhash);
        }
    }
}
