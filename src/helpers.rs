use std::{fs::File, io::{Read, Write}};
use serde_json::Value;
use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use hex_literal::hex;
use std::str;

use crate::PLUGIN_DIR;

pub fn get_js(name: &str) -> String {
    let mut file = File::open(format!("input/{}/{}.js", PLUGIN_DIR.get().unwrap(), name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents = contents.replace("\\n", " ");
    let re = regex::Regex::new(r"\s+").unwrap();
    re.replace_all(&contents, " ").to_string()
}
pub fn get_url(name: &str) -> String {
    let mut file = File::open(format!("input/{}/{}.txt", PLUGIN_DIR.get().unwrap(), name)).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}
pub fn write_js(data: Value) {
    println!("Creating json file...");
    let json_string = serde_json::to_string_pretty(&data).unwrap();
    std::fs::create_dir_all("output").unwrap();
    let mut file = File::create(format!("output/{}.json", PLUGIN_DIR.get().unwrap())).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}
pub async fn fetch(url: String) -> String {
    println!("Fetching...");
    let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()
        .await.unwrap();
    let mut data = response.text().await.unwrap();
    data = data.replace("\n", " ").replace('`', "").replace("${", "S").replace("\\\"", "'");
    let re = regex::Regex::new(r"\s+").unwrap();
    data = re.replace_all(&data, " ").to_string();
    data
}
pub async fn fetch_with_header(url: String, key: &str, val: &str) -> String {
    println!("Fetching...");
    let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
    let client = reqwest::Client::new();
    let response = client.get(url)
        .header(reqwest::header::USER_AGENT, user_agent)
        .header(key, val)
        .send()
        .await.unwrap();
    let mut data = response.text().await.unwrap();
    data = data.replace("\n", " ").replace('`', "").replace("${", "S").replace("\\\"", "'");
    let re = regex::Regex::new(r"\s+").unwrap();
    data = re.replace_all(&data, " ").to_string();
    data
}
pub async fn post_fetch(url: String) -> String {
    let user_agent = "Mozilla/5.0 (Linux; Android 13; SM-S901U) AppleWebkit/537.36 (KHTML, like Gecko Chrome/112.0.0.0 Mobile Safari/537.36";
    let client = reqwest::Client::new();
    let response = client.post(url)
      .header(reqwest::header::USER_AGENT, user_agent)
      .send()
      .await.unwrap();
    let mut data = response.text().await.unwrap();
    data = data.replace("\n", " ").replace('`', "").replace("${", "S").replace("\\\"", "'");
    let re = regex::Regex::new(r"\s+").unwrap();
    data = re.replace_all(&data, " ").to_string();
    data
}

pub fn text_to_byte_arr(input: &str) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for (i, c) in input.chars().enumerate() {
        result.push(c as u8);
    }
    result
}

type Aes256Cbc = Cbc<Aes256, Pkcs7>;
pub fn crypto_handler(input: &str, iv: &[u8], secret_key: &[u8], encrypt: bool) -> Result<String, Box<dyn std::error::Error>> {
    let cipher = Aes256Cbc::new_from_slices(secret_key, iv)?;

    if encrypt {
        let encrypted_data = cipher.encrypt_vec(input.as_bytes());
        Ok(base64::encode(&encrypted_data))
    } else {
        let decoded_data = base64::decode(input)?;
        let decrypted_data = cipher.decrypt_vec(&decoded_data)?;
        Ok(str::from_utf8(&decrypted_data)?.to_string())
    }
}