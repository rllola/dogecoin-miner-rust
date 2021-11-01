mod utils;
mod configs;
mod miner;
mod rpc;

fn main() {
    println!("Dogecoin miner!");
    
    let config = configs::read_config();
    
    if config.mergemining {
        println!("START MERGE MINE!");
        loop {
            miner::merge_mine(&config);
        }

    } else {
        loop {
            miner::mine(&config);
        }
    }    
}