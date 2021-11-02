use serde_json::json;

pub fn get_block_template(url: &String, user: &String, password: &String, params: Option<serde_json::Value>) -> serde_json::Value {

    let mut body = json!({
        "jsonrpc": "1.0",
        "id": "kek",
        "method": "getblocktemplate",
        "params": []
    });

    if params.is_some() {
        body = json!({
            "jsonrpc": "1.0",
            "id": "kek",
            "method": "getblocktemplate",
            "params": [params]
        });
    }
    
    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .basic_auth(user, Some(password))
        .json(&body)
        .send()
        .unwrap();
        
    let block_template: serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();

    return block_template
}

pub fn submit_block(block_hex: String, url: &String, user: &String, password: &String) -> serde_json::Value {
    let body_submit = json!({
        "jsonrpc": "1.0",
        "id": "kek",
        "method": "submitblock",
        "params": [block_hex]
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .basic_auth(user, Some(password))
        .json(&body_submit)
        .send()
        .unwrap();
            
    let answer : serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();

    return answer;
}

pub fn submit_aux_block(block_hash: String, block_hex: String, url: &String, user: &String, password: &String) -> serde_json::Value {
    let body_submit = json!({
        "jsonrpc": "1.0",
        "id": "kek",
        "method": "getauxblock",
        "params": [block_hash, block_hex]
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .basic_auth(user, Some(password))
        .json(&body_submit)
        .send()
        .unwrap();
            
    let answer : serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();

    return answer;
}

pub fn get_aux_block(url: &String, user: &String, password: &String) -> serde_json::Value {
    let body_submit = json!({
        "jsonrpc": "1.0",
        "id": "kek",
        "method": "getauxblock",
        "params": []
    });

    let client = reqwest::blocking::Client::new();
    let res = client.post(url)
        .basic_auth(user, Some(password))
        .json(&body_submit)
        .send()
        .unwrap();
            
    let answer : serde_json::Value = serde_json::from_str(&res.text().unwrap()).unwrap();

    return answer;
}