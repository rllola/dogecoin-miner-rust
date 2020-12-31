// Way faster with c bindings version !!!
use rust_scrypt;

pub fn double_hash_256(message: &Vec<u8>) -> [u8;32] {
    let tmp = hmac_sha256::Hash::hash(&message);
    
    hmac_sha256::Hash::hash(&tmp)
}

pub fn calculate_merkle_root(merkle_tree: &Vec<[u8;32]>) -> [u8;32] {
    let mut tmp : Vec<[u8;32]> = Vec::new();
    tmp.extend_from_slice(merkle_tree);
    
    while tmp.len() > 1 {
        let mut new_merkle_tree : Vec<[u8;32]> = Vec::new();
        
        if tmp.len() % 2 == 1 {
            let mut last : [u8;32] = [0;32];
            last.copy_from_slice(tmp.last().unwrap());
            tmp.push(last);
        }
        
        for x in (0..tmp.len()).step_by(2) {
            let mut concat_hash : Vec<u8> = Vec::new();
            concat_hash.extend_from_slice(&tmp[x]);
            concat_hash.extend_from_slice(&tmp[x+1]);
            new_merkle_tree.push(double_hash_256(&concat_hash));
        }
        
        tmp = Vec::new();
        tmp.extend_from_slice(&new_merkle_tree);
    }
    
    tmp[0]
}

pub fn compact_size(size: usize) -> Vec<u8> {
    let mut cs : Vec<u8> = Vec::new();
    
    if size <= 252 {
        cs.push(size as u8);
        return cs;
    } else if size <= 65535 {
        cs.push(253_u8);
        cs.extend_from_slice(&(size as u16).to_le_bytes());
        return cs;
    } else if size <= 4294967295 {
        cs.push(254_u8);
        cs.extend_from_slice(&(size as u32).to_le_bytes());
        return cs;
    } else {
        cs.push(255_u8);
        cs.extend_from_slice(&(size as u64).to_le_bytes());
        return cs;
    }
}

pub fn scrypt_hasher(block: &Vec<u8>) -> [u8;32] {
    let mut scrypt_hash : [u8;32] = [0;32];

    // Got here https://litecoin.info/index.php/Scrypt
    // N = 1024
    let params = rust_scrypt::ScryptParams::new(1024, 1, 1);
    rust_scrypt::scrypt(&block, &block, &params, &mut scrypt_hash);
    // REVIEW : need to reverse it ?
    scrypt_hash.reverse();
    
    scrypt_hash
}

#[cfg(test)]
mod tests {
    use super::{calculate_merkle_root, scrypt_hasher, double_hash_256};
    
    #[test]
    fn test_calculate_merkle_root() {
        // https://learnmeabitcoin.com/technical/merkle-root
        let hashes : Vec<String> = vec![
            "8c14f0db3df150123e6f3dbbf30f8b955a8249b62ac1d1ff16284aefa3d06d87".into(),
            "fff2525b8931402dd09222c50775608f75787bd2b87e56995a7bdd30f79702c4".into(),
            "6359f0868171b1d194cbee1af2f16ea598ae8fad666d9b012c8ed2b79a236ec4".into(),
            "e9a66845e05d5abc0ad04ec80f774a7e585c6e8db975962d069a522137b80c1d".into(),
        ];
        let expected_merkle_root = "f3e94742aca4b5ef85488dc37c06c3282295ffec960994b2c0d5ac2a25a95766";
        
        let mut merkle_tree : Vec<[u8;32]> = Vec::new();
        for txid in hashes {
            let mut tmp : [u8;32] = [0;32];
            let mut hash = hex::decode(&txid).unwrap();
            hash.reverse();
            tmp.copy_from_slice(&hash);
            merkle_tree.push(tmp);
        }
        
        let mut merkle_root = calculate_merkle_root(&merkle_tree);
        merkle_root.reverse();
        assert_eq!(expected_merkle_root, hex::encode(merkle_root));
    }
    
    #[test]
    fn test_calculate_merkle_root_2() {
        // https://sochain.com/api/v2/get_block/DOGE/a6fe70118546f7f4a933aec31d995e2d7511999a53eeaa7eb9cd97cb77257210
        let hashes : Vec<String> = vec![
            "9cb7e337f2fbedf341c665fff53780f80ac584a0f8f84e9d3349168e699b1f4a".into(),
            "ca8e9210796796d8626865e4d9260a02f505e5e8cd2e64c40703c0b83b212cc5".into(),
            "24aeebf69d8defb850be4f0d33a7a9a77b0cd2db7b18f40b774d981869c43861".into(),
            "9aad747f79b7da57b98f10ab43b38258710f2e0f513d97dee93915b55e994af5".into(),
            "4ddc7177859633e34540d4f82cd2963454c9f545ae6a6c3e536050d5ec819ef3".into(),
            "b5d41d003fcbdc075eab7507462e38cef23fe511dff5aa4c0c01e2dcd1f0b4af".into(),
            "2e1c73f4456379866816f245f3b7d48683f8f3164ea9846102525454e30fdbfa".into(),
            "814acbeb140e6f0b419feaf5586645460d25658d063a055a5528cdff5a2c2697".into(),
            "90b9f6e0f9651f518a3b227b3e0d5764aab6220e464fcd21266c192c755b07b1".into(),
        ];
        let expected_merkle_root = "60e96914503b6fba05feb27c343263471e8da0d855665f80c0f65a5cc0c6e6fd";
            
        let mut merkle_tree : Vec<[u8;32]> = Vec::new();
        for txid in hashes {
            let mut tmp : [u8;32] = [0;32];
            let mut hash = hex::decode(&txid).unwrap();
            hash.reverse();
            tmp.copy_from_slice(&hash);
            merkle_tree.push(tmp);
        }
            
        let mut merkle_root = calculate_merkle_root(&merkle_tree);
        merkle_root.reverse();
        assert_eq!(expected_merkle_root, hex::encode(merkle_root));
    }
    
    #[test]
    fn test_calculate_merkle_root_3() {
        let hashes : Vec<String> = vec![
            "a335b243f5e343049fccac2cf4d70578ad705831940d3eef48360b0ea3829ed4".into(),
            "d5fd11cb1fabd91c75733f4cf8ff2f91e4c0d7afa4fd132f792eacb3ef56a46c".into(),
            "0441cb66ef0cbf78c9ecb3d5a7d0acf878bfdefae8a77541b3519a54df51e7fd".into(),
            "1a8a27d690889b28d6cb4dacec41e354c62f40d85a7f4b2d7a54ffc736c6ff35".into(),
            "1d543d550676f82bf8bf5b0cc410b16fc6fc353b2a4fd9a0d6a2312ed7338701".into(),
        ];
        let expected_merkle_root = "5766798857e436d6243b46b5c1e0af5b6806aa9c2320b3ffd4ecff7b31fd4647";
                
        let mut merkle_tree : Vec<[u8;32]> = Vec::new();
        for txid in hashes {
            let mut tmp : [u8;32] = [0;32];
            let mut hash = hex::decode(&txid).unwrap();
            hash.reverse();
            tmp.copy_from_slice(&hash);
            merkle_tree.push(tmp);
        }
                
        let mut merkle_root = calculate_merkle_root(&merkle_tree);
        merkle_root.reverse();
        assert_eq!(expected_merkle_root, hex::encode(merkle_root));
    }
    
    #[test]
    fn test_scrypt_hasher() {
        let block = hex::decode("01000000f615f7ce3b4fc6b8f61e8f89aedb1d0852507650533a9e3b10b9bbcc30639f279fcaa86746e1ef52d3edb3c4ad8259920d509bd073605c9bf1d59983752a6b06b817bb4ea78e011d012d59d4".to_string()).unwrap();
        
        let scrypt_hash = scrypt_hasher(&block);
                
        assert_eq!(hex::encode(scrypt_hash), "0000000110c8357966576df46f3b802ca897deb7ad18b12f1c24ecff6386ebd9");
    }
    
    #[test]
    fn test_scrypt_hasher_2() {
        let block = hex::decode("040062002385f417d64d2b45597c23cac28c9a7cf0a3ffedbc54f7a926775a0956415d66c5e6b1d78037d8debd6337f4b7786126cb98f96f2e259586073bf235867f35e3da14ea5fffff0f1e13d40000".to_string()).unwrap();
            
        let scrypt_hash = scrypt_hasher(&block);
                                    
        assert_eq!(hex::encode(scrypt_hash), "00000b4bdd2b7681ea81fe1f060f1306a516f726f01ddbc07d81171e8c697f2c");
    }
    
    #[test]
    fn test_scrypt_hasher_3() {
        let block = hex::decode("0400620023fc98ec01e449177ac7006d5fc8a857354adbfaa37d026b95eee134f68f3a994fe1a80078f703f20db31e18aee9a103f503c72a32bb76729baf2d7db1ef27aa7a5aeb5fffff0f1e3d090000".to_string()).unwrap();
            
        let scrypt_hash = scrypt_hasher(&block);
        
        if scrypt_hash[0] > 0 || scrypt_hash[1] > 0 || scrypt_hash[2] > 15 {
            assert!(false);
        }
                                   
        assert_eq!(hex::encode(scrypt_hash), "00000a5c0c8833a6feb840ea807d98f0b479a4813687fd9189c728819e1a5821");
    }
    
    #[test]
    fn test_scrypt_hasher_4() {
        let mut block = hex::decode("040062005d60312bdeefd01a11f9b4275f8491c542d725bf67dd8dbac4bfa096d09332c58374018853e36fca3aec59e6d2f800f4db0e7e9f7f60ad2ec40131793742113940e0e95fffff0f1ee4f00000".to_string()).unwrap();
        let nonce = 62436_u32;
        
        block.truncate(block.len()-4);
        block.extend_from_slice(&nonce.to_le_bytes());
        
        let scrypt_hash = scrypt_hasher(&block);
            
        if scrypt_hash[0] > 0 || scrypt_hash[1] > 0 || scrypt_hash[2] > 15 {
            assert!(false);
        }
                                       
        assert_eq!(hex::encode(scrypt_hash), "0000050e6c273d00bb83f38b3f22a9a6b2a22246949cf8fc0f756e13716bf5c3");
    }

    #[test]
    fn test_calculate_merkle_root_4() {
        let hashes : Vec<String> = vec![
            "a8360edc1c42bbfabe57fff800cc74654527d5a7cf43630d66f830c2b5e41a66".into(),
            "f0521f0e4626a94bb67a14a9714903c06a6bbf972c9682c3321bbfdeaa64f6d8".into(),
        ];
        let expected_merkle_root = "f4291128de79e485d7a470f219183cb330d2c8177823db26f6065e1a4817a87e";
                
        let mut merkle_tree : Vec<[u8;32]> = Vec::new();
        for txid in hashes {
            let mut tmp : [u8;32] = [0;32];
            let mut hash = hex::decode(&txid).unwrap();
            hash.reverse();
            tmp.copy_from_slice(&hash);
            merkle_tree.push(tmp);
        }
                
        let mut merkle_root = calculate_merkle_root(&merkle_tree);
        merkle_root.reverse();
        assert_eq!(expected_merkle_root, hex::encode(merkle_root));                        
    }
    
    #[test]
    fn test_calculate_txid() {
        let coinbase_tx = hex::decode("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff060398292e0105ffffffff010010a5d4e80000001976a9144e810ea0600b308d58589a7d2df76317dfe6b5cf88ac00000000").unwrap();
        let mut txid = double_hash_256(&coinbase_tx);
        txid.reverse();
                
        assert_eq!(hex::encode(txid), "39114237793101c42ead607f9f7e0edbf400f8d2e659ec3aca6fe35388017483");
    }
    
}