use crate::fce_results::{JsonRpcResult, JsonRpcTransactionResult};
use marine_rs_sdk::marine;
use std::sync::atomic::{AtomicUsize, Ordering};
use tiny_keccak::{Hasher, Keccak};

pub const BLOCK_NUMBER_TAGS: [&'static str; 3] = ["latest", "earliest", "pending"];
pub static NONCE_COUNTER: AtomicUsize = AtomicUsize::new(1);

pub fn get_nonce() -> u64 {
    NONCE_COUNTER.fetch_add(1, Ordering::SeqCst) as u64
}

pub fn check_response_string(response: String, id: &u64) -> JsonRpcResult {
    if response.len() == 0 {
        let err_msg = "{\"jsonrpc\":\"$V\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
        let err_msg = err_msg.replace("$ID", &id.to_string());
        return JsonRpcResult::from_res(Result::from(Err(err_msg)));
    }

    match response.contains("error") {
        true => JsonRpcResult::from_res(Result::from(Err(response))),
        false => JsonRpcResult::from_res(Result::from(Ok(response))),
    }
}

pub fn check_response_transaction_string(response: String, id: &u64) -> JsonRpcTransactionResult {
    if response.len() == 0 {
        let err_msg = "{\"jsonrpc\":\"$V\",\"id\":$ID,\"error\":{\"code\":-32700,\"message\":Curl connection failed}}";
        let err_msg = err_msg.replace("$ID", &id.to_string());
        return JsonRpcTransactionResult::from_res(Result::from(Err(err_msg)));
    }

    match response.contains("error") {
        true => JsonRpcTransactionResult::from_res(Result::from(Err(response))),
        false => JsonRpcTransactionResult::from_res(Result::from(Ok(response))),
    }
}

pub fn wei_to_eth(amount: &u128) -> f64 {
    *amount as f64 / (1_000_000_000.0 * 1_000_000_000.0)
}

#[marine]
pub fn hex_to_decimal(hex: String) -> u64 {
    u64::from_str_radix(&hex[2..], 16).unwrap()
}

#[marine]
pub fn decimal_to_hex(decimal: u64) -> String {
    format!("0x{:x}", decimal)
}

#[marine]
pub fn util_get_method_hash(input: String) -> String {
    let input_str = input.as_str();
    let input = input_str.strip_prefix("0x").unwrap_or(input_str);
    let input_bytes = hex::decode(input).unwrap();
    let b = &input_bytes[0..4];
    format!("0x{}", hex::encode(&b))
}

pub fn pk_to_add(pk_string: String) -> String {
    let pk_bytes = hex::decode(pk_string[2..].to_string()).unwrap();
    let mut hasher = Keccak::v256();
    hasher.update(&pk_bytes);
    let mut add_bytes: [u8; 32] = Default::default();
    hasher.finalize(&mut add_bytes);
    let add_string_suf = hex::encode(&add_bytes[12..]);
    let mut add_string = "0x".to_string();
    add_string.push_str(&add_string_suf);
    add_string
}
