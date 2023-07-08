mod configs;
mod miner;
mod rpc;
mod utils;

fn main() {
    println!("Dogecoin miner!");

    let config = configs::read_config();

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
