use std::fs::File;
use std::io::prelude::*;
use serde_json::json;
use std::vec::Vec;
use std::time;

mod utils;
use crate::utils::{compact_size, calculate_merkle_root, scrypt_hasher, double_hash_256};

fn main() {
    println!("Dogecoin miner!");
    
    let mut file = File::open(".env").expect(".env file required");
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents.lines();
    let values = contents.lines().collect::<Vec<&str>>();
    
    println!("{:?}", values);
    
    assert_eq!(values.len(), 4);
    
    let host = values[0];
    let port = values[1].parse::<i32>().unwrap();
    let user = values[2];
    let password = values[3];
    
    while true {
        let url = format!("http://{}:{}", host, port);
        let body = json!({
            "jsonrpc": "1.0",
            "id": "kek",
            "method": "getblocktemplate",
            "params": []
        });
        
        let client = reqwest::blocking::Client::new();
        let res = client.post(&url)
            .basic_auth(user, Some(password))
            .json(&body)
            .send()
            .unwrap();
            
        let block_template: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
        
        /* Create coinbase transaction */
        let mut coinbase_tx : Vec<u8> = Vec::new();
        let version = hex::decode("01000000").unwrap();
        coinbase_tx.extend_from_slice(&version);
        
        // Pushing number of inputs
        coinbase_tx.push(1);
        
        let hash = hex::decode("0000000000000000000000000000000000000000000000000000000000000000").unwrap();
        coinbase_tx.extend_from_slice(&hash);
        
        let index = hex::decode("ffffffff").unwrap();
        coinbase_tx.extend_from_slice(&index);
        
        let height = block_template["result"]["height"].as_u64().unwrap();
        let height_bytes = height.to_le_bytes();
        /* For now we take length 3. Need to be improved.*/
        let mut height_bytes_varint = vec![0;3];
        height_bytes_varint.copy_from_slice(&height_bytes[0..3]);

        // Arbitrary data
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
        
        // Send to nbMFaHF9pjNoohS4fD1jefKBgDnETK9uPu
        let script_out_data = hex::decode("76a9144e810ea0600b308d58589a7d2df76317dfe6b5cf88ac").unwrap();
        coinbase_tx.push(script_out_data.len() as u8);
        coinbase_tx.extend_from_slice(&script_out_data);
        
        let locktime = hex::decode("00000000").unwrap();
        coinbase_tx.extend_from_slice(&locktime);
        
        /* Done ! */
        
        /* Calculate txid */
        let coinbase_txid = double_hash_256(&coinbase_tx);
        
        /* Create merkleroot */
        let mut merkle_tree : Vec<[u8;32]> = Vec::new();
        
        merkle_tree.push(coinbase_txid);

        for x in block_template["result"]["transactions"].as_array().unwrap().iter() {
            let mut txid : [u8;32] = [0; 32];
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
                
        /* Create block */
        let mut block: Vec<u8> = Vec::new();
        let version = block_template["result"]["version"].as_u64().unwrap();
        block.extend_from_slice(&(version as u32).to_le_bytes());
        
        // Add previous header
        let mut previousblockhash = hex::decode(block_template["result"]["previousblockhash"].as_str().unwrap()).unwrap();
        previousblockhash.reverse();
        block.extend_from_slice(&previousblockhash);
        
        // Add Merkle Root !
        block.extend_from_slice(&merkle_root);
        
        // Time (Unix epoch time). There is a 2 hours thing future...
        // Could use "curtime"
        let now = time::SystemTime::now().duration_since(time::SystemTime::UNIX_EPOCH).unwrap().as_secs();
        let timestamp = (now as u32).to_le_bytes();
        block.extend_from_slice(&timestamp);

        // nBits    
        let mut nbits = hex::decode(block_template["result"]["bits"].as_str().unwrap()).unwrap();
        nbits.reverse();
        block.extend_from_slice(&nbits);
        
        let mut nonce = 0_u32;
        block.extend_from_slice(&nonce.to_le_bytes());
        
        //let count = buffer.iter().rev().take_while(|b| **b == 0).count();
        let _target = hex::decode(block_template["result"]["target"].as_str().unwrap()).unwrap();
        
        let mut scrypt_hash : [u8;32] = [255;32];
        
        // Could do better here
        while scrypt_hash[0] > 0 || scrypt_hash[1] > 0 || scrypt_hash[2] > 15 {       
            nonce = nonce + 1;
            
            block.truncate(block.len()-4);
            block.extend_from_slice(&nonce.to_le_bytes());

            scrypt_hash = scrypt_hasher(&block);
        }
        
        println!("Number of attempts : {:?}", nonce);
        println!("{:?}", hex::encode(scrypt_hash));
        println!("{:?}", block_template["result"]["target"].as_str().unwrap());        
        
        // We hash the header... but we need this to submit block
        let len_transactions = compact_size(block_template["result"]["transactions"].as_array().unwrap().len()+1);
        block.extend_from_slice(&len_transactions);
        
        // Add coinbase
        block.extend_from_slice(&coinbase_tx);
            
        for x in block_template["result"]["transactions"].as_array().unwrap().iter() {
            block.extend_from_slice(&hex::decode(x["data"].as_str().unwrap()).unwrap());
        }
        
        let block_hex : String = hex::encode(block);
        
        println!("{:?}", block_hex);
        
        let body_submit = json!({
            "jsonrpc": "1.0",
            "id": "kek",
            "method": "submitblock",
            "params": [block_hex]
        });
        
            
        let client = reqwest::blocking::Client::new();
        let res = client.post(&url)
            .basic_auth(user, Some(password))
            .json(&body_submit)
            .send()
            .unwrap();
                
        let answer : serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();
        println!("{:?}", answer);    
    }
    
}