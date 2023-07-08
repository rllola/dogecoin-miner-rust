use serde_json::json;
use std::time;
use std::vec::Vec;

use crate::configs::Config;
use crate::rpc;
use crate::utils::{
    calculate_merkle_root, coinbase_merkle_links, compact_size, double_hash_256, scrypt_hasher,
};

/* Create coinbase transaction */
fn create_coinbase_tx(block_template: &serde_json::Value, pubkeyhash: &Vec<u8>) -> Vec<u8> {
    let mut coinbase_tx: Vec<u8> = Vec::new();
    let version = hex::decode("01000000").unwrap();
    coinbase_tx.extend_from_slice(&version);

    // Pushing number of inputs
    coinbase_tx.push(1);

    let hash =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    coinbase_tx.extend_from_slice(&hash);

    let index = hex::decode("ffffffff").unwrap();
    coinbase_tx.extend_from_slice(&index);

    let height = block_template["result"]["height"].as_u64().unwrap();
    let height_bytes = height.to_le_bytes();
    /* For now we take length 3. Need to be improved.*/
    let mut height_bytes_varint = vec![0; 3];
    height_bytes_varint.copy_from_slice(&height_bytes[0..3]);

    // Arbitrary data
    /* TODO: arbitrary data in config file */
    let script_data = hex::decode("4c6f6c6120697320746865206265737421").unwrap();

    // Adding script length
    coinbase_tx.push((4 + script_data.len()) as u8);
    // Adding height data length
    coinbase_tx.push(3);
    // Adding height data
    coinbase_tx.extend_from_slice(&height_bytes_varint);
    // Adding script data
    coinbase_tx.extend_from_slice(&script_data);

    let sequence = hex::decode("ffffffff").unwrap();
    coinbase_tx.extend_from_slice(&sequence);

    // Now define output
    coinbase_tx.push(1);

    let value = block_template["result"]["coinbasevalue"].as_i64().unwrap();
    coinbase_tx.extend_from_slice(&value.to_le_bytes());

    /* 76a914<pubkeyhash>88ac */
    let mut script_out_data = vec![118, 169, 20];
    script_out_data.extend_from_slice(pubkeyhash);
    script_out_data = [script_out_data, vec![136, 172]].concat();

    coinbase_tx.push(script_out_data.len() as u8);
    coinbase_tx.extend_from_slice(&script_out_data);

    let locktime = hex::decode("00000000").unwrap();
    coinbase_tx.extend_from_slice(&locktime);

    return coinbase_tx;
}

/* Create block header */
fn create_blockheader(block_template: &serde_json::Value, merkle_root: [u8; 32]) -> Vec<u8> {
    let mut blockheader: Vec<u8> = Vec::new();
    let version = block_template["result"]["version"].as_u64().unwrap();
    blockheader.extend_from_slice(&(version as u32).to_le_bytes());

    // Add previous header
    let mut previousblockhash = hex::decode(
        block_template["result"]["previousblockhash"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    previousblockhash.reverse();
    blockheader.extend_from_slice(&previousblockhash);

    // Add Merkle Root !
    blockheader.extend_from_slice(&merkle_root);

    // Time (Unix epoch time). There is a 2 hours thing future...
    // Could use "curtime"
    let now = time::SystemTime::now()
        .duration_since(time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let timestamp = (now as u32).to_le_bytes();
    blockheader.extend_from_slice(&timestamp);

    // nBits
    let mut nbits = hex::decode(block_template["result"]["bits"].as_str().unwrap()).unwrap();
    nbits.reverse();
    blockheader.extend_from_slice(&nbits);

    let nonce = 0_u32;
    blockheader.extend_from_slice(&nonce.to_le_bytes());

    return blockheader;
}

pub fn mine(config: &Config, pubkeyhash: &Vec<u8>) {
    let dogecoin_url = format!("http://{}:{}", config.dogecoin.ip, config.dogecoin.port);

    // Get block template from node
    let block_template: serde_json::Value = rpc::get_block_template(
        &dogecoin_url,
        &config.dogecoin.user,
        &config.dogecoin.password,
        None,
    );

    let coinbase_tx = create_coinbase_tx(&block_template, pubkeyhash);

    /* Calculate txid */
    let coinbase_txid = double_hash_256(&coinbase_tx);

    /* Create merkleroot */
    let mut merkle_tree: Vec<[u8; 32]> = Vec::new();

    merkle_tree.push(coinbase_txid);

    for x in block_template["result"]["transactions"]
        .as_array()
        .unwrap()
        .iter()
    {
        let mut txid: [u8; 32] = [0; 32];
        txid.copy_from_slice(&hex::decode(x["txid"].as_str().unwrap()).unwrap());
        txid.reverse();
        merkle_tree.push(txid);
    }

    /* Calculate merkleroot */
    // IT IS THE COINBASE TXID IF THERE IS NO TRANSACTIONS
    let mut merkle_root = coinbase_txid;

    if merkle_tree.len() > 1 {
        merkle_root = calculate_merkle_root(&merkle_tree);
    }

    let mut block = create_blockheader(&block_template, merkle_root);

    //let count = buffer.iter().rev().take_while(|b| **b == 0).count();
    let target = hex::decode(block_template["result"]["target"].as_str().unwrap()).unwrap();

    let mut scrypt_hash: [u8; 32] = [255; 32];

    // Could do better here
    let mut nonce = 0_u32;
    while hex::encode(&scrypt_hash) > hex::encode(&target) {
        nonce = nonce + 1;

        block.truncate(block.len() - 4);
        block.extend_from_slice(&nonce.to_le_bytes());

        scrypt_hash = scrypt_hasher(&block);
    }

    println!("Number of attempts : {:?}", nonce);
    println!("{:?}", hex::encode(scrypt_hash));
    println!("{:?}", block_template["result"]["target"].as_str().unwrap());

    // We hash the header... but we need this to submit block
    let len_transactions = compact_size(
        block_template["result"]["transactions"]
            .as_array()
            .unwrap()
            .len()
            + 1,
    );
    block.extend_from_slice(&len_transactions);

    // Add coinbase
    block.extend_from_slice(&coinbase_tx);

    for x in block_template["result"]["transactions"]
        .as_array()
        .unwrap()
        .iter()
    {
        block.extend_from_slice(&hex::decode(x["data"].as_str().unwrap()).unwrap());
    }

    let block_hex: String = hex::encode(block);

    println!("{:?}", block_hex);

    let answer = rpc::submit_block(
        block_hex,
        &dogecoin_url,
        &config.dogecoin.user,
        &config.dogecoin.password,
    );

    println!("{:?}", answer);
}

pub fn merge_mine(config: &Config, pubkeyhash: &Vec<u8>) {
    let dogecoin_url = format!("http://{}:{}", config.dogecoin.ip, config.dogecoin.port);
    let litecoin_config = config.litecoin.as_ref().unwrap();
    let litecoin_url = format!("http://{}:{}", litecoin_config.ip, litecoin_config.port);

    let dogecoin_aux_block_template: serde_json::Value = rpc::get_aux_block(
        &dogecoin_url,
        &config.dogecoin.user,
        &config.dogecoin.password,
    );

    /* Create Dogecoin coinbase transaction */
    let dogecoin_coinbase_tx = create_coinbase_tx(&dogecoin_aux_block_template, pubkeyhash);
    /* Calculate Dogecoin txid */
    let _dogecoin_coinbase_txid = double_hash_256(&dogecoin_coinbase_tx);
    let dogecoin_sha256_hash = hex::decode(
        &dogecoin_aux_block_template["result"]["hash"]
            .as_str()
            .unwrap(),
    )
    .unwrap();

    /*
        PREPARE LITECOIN BLOCK
    */

    let litecoin_block_template: serde_json::Value = rpc::get_block_template(
        &litecoin_url,
        &litecoin_config.user,
        &litecoin_config.password,
        Some(json!({"rules": ["segwit"]})),
    );
    println!("{:?}", litecoin_block_template);

    /* Create parent block coinbase transaction */
    let mut litecoin_coinbase_tx: Vec<u8> = Vec::new();
    // TODO: using version from get_block_template
    let version = hex::decode("01000000").unwrap();
    litecoin_coinbase_tx.extend_from_slice(&version);

    // Pushing number of inputs
    litecoin_coinbase_tx.push(1);

    let hash =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
    litecoin_coinbase_tx.extend_from_slice(&hash);

    let index = hex::decode("ffffffff").unwrap();
    litecoin_coinbase_tx.extend_from_slice(&index);

    let height = litecoin_block_template["result"]["height"]
        .as_u64()
        .unwrap();
    let height_bytes = height.to_le_bytes();
    /* For now we take length 3. Need to be improved.*/
    let mut height_bytes_varint = vec![0; 3];
    height_bytes_varint.copy_from_slice(&height_bytes[0..3]);

    // Specific data
    let mut script_data: Vec<u8> = Vec::new();

    let auxpow_magic_bytes = hex::decode("fabe6d6d").unwrap();
    // It is 1 in size because we only merge mine dogecoin and merkle nonce is 0
    let merkle_auxpow_blob_data = hex::decode("0100000000000000").unwrap();

    script_data.extend_from_slice(&auxpow_magic_bytes);
    // IMPORTANT! Dogecoin sha256 shouldn't be reversed when get from `getauxblock` otherwise it returns false.
    script_data.extend_from_slice(&dogecoin_sha256_hash);
    script_data.extend_from_slice(&merkle_auxpow_blob_data);

    // Adding script length
    litecoin_coinbase_tx.push((4 + script_data.len() + 1) as u8);
    // Adding height data length
    litecoin_coinbase_tx.push(3);
    // Adding height data
    litecoin_coinbase_tx.extend_from_slice(&height_bytes_varint);
    // Adding the script data size
    litecoin_coinbase_tx.push((script_data.len()) as u8);
    // Adding script data
    litecoin_coinbase_tx.extend_from_slice(&script_data);

    let sequence = hex::decode("ffffffff").unwrap();
    litecoin_coinbase_tx.extend_from_slice(&sequence);

    // Now define output
    litecoin_coinbase_tx.push(1);

    let value = litecoin_block_template["result"]["coinbasevalue"]
        .as_i64()
        .unwrap();
    litecoin_coinbase_tx.extend_from_slice(&value.to_le_bytes());

    // Send to nbMFaHF9pjNoohS4fD1jefKBgDnETK9uPu
    /* TODO: address in config file */
    let script_out_data =
        hex::decode("76a9144e810ea0600b308d58589a7d2df76317dfe6b5cf88ac").unwrap();
    litecoin_coinbase_tx.push(script_out_data.len() as u8);
    litecoin_coinbase_tx.extend_from_slice(&script_out_data);

    let locktime = hex::decode("00000000").unwrap();
    litecoin_coinbase_tx.extend_from_slice(&locktime);

    /* Calculate txid */
    let litecoin_coinbase_txid = double_hash_256(&litecoin_coinbase_tx);

    /* Create merkleroot */
    let mut litecoin_merkle_tree: Vec<[u8; 32]> = Vec::new();

    litecoin_merkle_tree.push(litecoin_coinbase_txid);

    for x in litecoin_block_template["result"]["transactions"]
        .as_array()
        .unwrap()
        .iter()
    {
        let mut txid: [u8; 32] = [0; 32];
        txid.copy_from_slice(&hex::decode(x["txid"].as_str().unwrap()).unwrap());
        txid.reverse();
        litecoin_merkle_tree.push(txid);
    }

    /* Calculate merkleroot */
    let mut litecoin_merkle_root = litecoin_coinbase_txid;

    if litecoin_merkle_root.len() > 1 {
        litecoin_merkle_root = calculate_merkle_root(&litecoin_merkle_tree);
    }

    /* Calculate PoW for litecoin */

    let mut litecoin_blockheader =
        create_blockheader(&litecoin_block_template, litecoin_merkle_root);

    let litecoin_target = hex::decode(
        litecoin_block_template["result"]["target"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    let mut dogecoin_target = hex::decode(
        dogecoin_aux_block_template["result"]["target"]
            .as_str()
            .unwrap(),
    )
    .unwrap();
    // ONLY REVERVE FOR AUX BLOCK
    dogecoin_target.reverse();

    let mut litecoin_scrypt_hash: [u8; 32] = [255; 32];

    let mut nonce = 0_u32;
    while hex::encode(&litecoin_scrypt_hash) > hex::encode(&litecoin_target)
        || hex::encode(&litecoin_scrypt_hash) > hex::encode(&dogecoin_target)
    {
        nonce = nonce + 1;

        litecoin_blockheader.truncate(litecoin_blockheader.len() - 4);
        litecoin_blockheader.extend_from_slice(&nonce.to_le_bytes());

        litecoin_scrypt_hash = scrypt_hasher(&litecoin_blockheader);
    }

    let mut litecoin_block: Vec<u8> = Vec::new();
    litecoin_block.extend_from_slice(&litecoin_blockheader);

    let litecoin_sha256_hash = double_hash_256(&litecoin_blockheader);

    println!("FOUND ! {:?}", hex::encode(&litecoin_sha256_hash));

    if hex::encode(&litecoin_scrypt_hash) < hex::encode(&litecoin_target) {
        println!("Found Litecoin valid block");

        // We hash the header... but we need this to submit block
        let len_transactions = compact_size(
            litecoin_block_template["result"]["transactions"]
                .as_array()
                .unwrap()
                .len()
                + 1,
        );
        litecoin_block.extend_from_slice(&len_transactions);

        // Add coinbase
        litecoin_block.extend_from_slice(&litecoin_coinbase_tx);

        for x in litecoin_block_template["result"]["transactions"]
            .as_array()
            .unwrap()
            .iter()
        {
            litecoin_block.extend_from_slice(&hex::decode(x["data"].as_str().unwrap()).unwrap());
        }

        let block_hex: String = hex::encode(litecoin_block);

        println!("{:?}", block_hex);

        let answer = rpc::submit_block(
            block_hex,
            &litecoin_url,
            &litecoin_config.user,
            &litecoin_config.password,
        );

        println!("{:?}", answer);
    }

    println!("Found Dogecoin valid block");

    let mut dogecoin_block: Vec<u8> = Vec::new();

    // Extend with litecoin coinbase transaction
    dogecoin_block.extend_from_slice(&litecoin_coinbase_tx);

    // Litecoin block header hash
    dogecoin_block.extend_from_slice(&litecoin_sha256_hash);

    let mut merkle_links: Vec<[u8; 32]> = coinbase_merkle_links(&litecoin_merkle_tree);
    // Remove first one which is coinbase tx id
    merkle_links.remove(0);

    let len_merkle_links = compact_size(merkle_links.len());
    dogecoin_block.extend_from_slice(&len_merkle_links);

    for hash in merkle_links.iter() {
        dogecoin_block.extend_from_slice(hash);
    }

    let bitmask = hex::decode("00000000").unwrap();
    dogecoin_block.extend_from_slice(&bitmask);

    // Aux blockchain links
    let aux_blockchain_bitmask = hex::decode("0000000000").unwrap();
    dogecoin_block.extend_from_slice(&aux_blockchain_bitmask);

    // Adding litecoin block hearder
    dogecoin_block.extend_from_slice(&litecoin_blockheader);

    let block_hex: String = hex::encode(&dogecoin_block);

    let block_id: String = dogecoin_aux_block_template["result"]["hash"]
        .as_str()
        .unwrap()
        .to_string();

    println!("{:?}", block_id);
    println!("{:?}", block_hex);

    // AuxPow Block can only be submitted using getauxblock ?
    let answer = rpc::submit_aux_block(
        block_id,
        block_hex,
        &dogecoin_url,
        &config.dogecoin.user,
        &config.dogecoin.password,
    );

    println!("{:?}", answer);
}
