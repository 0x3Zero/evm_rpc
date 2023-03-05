use std::str::FromStr;

use crate::{
    curl_request_res,
    eth_calls::eth_call,
    models::log_param::{DataLogParam, EventLogParamResult},
    types::{TxCall, TxLog},
};
use ethabi::{Contract, RawLog, Token};
use ethereum_types::{H160, H256, U256};
use marine_rs_sdk::marine;

#[marine]
#[derive(Debug)]
pub struct TxParam {
    value_type: String,
    value: String,
}

#[marine]
pub fn contract_view_call(
    node_url: String,
    abi_url: String,
    method_name: String,
    contract_address: String,
    tx_params: Vec<TxParam>,
) -> String {
    let args = vec![format!(r#"{}"#, abi_url)];
    let response = curl_request_res(args).unwrap();
    let contract = Contract::load(response.as_bytes()).unwrap();
    let func = contract.function(&method_name).unwrap();

    let tokens: Vec<Token> = tx_params
        .into_iter()
        .map(|param| match param.value_type.as_ref() {
            "address" => Token::Address(H160::from_str(&param.value).unwrap()),
            "uint" => Token::Uint(U256::from_dec_str(&param.value).unwrap()),
            _ => Token::String(param.value),
        })
        .collect();

    let data_in_bytes = func.encode_input(tokens.as_slice()).unwrap();

    let params = TxCall {
        to: Some(H160::from_str(&contract_address).unwrap()),
        data: Some(data_in_bytes.into()),
        ..Default::default()
    };

    let response = eth_call(node_url, params, "latest".into()).result;
    response
}

/**
 * Decode logs individually
 */
#[marine]
pub fn decode_logs(abi_url: String, topics: Vec<String>, data: String) -> EventLogParamResult {
    let args = vec![format!(r#"{}"#, abi_url)];
    let response = curl_request_res(args).unwrap();
    let contract = Contract::load(response.as_bytes()).unwrap();

    decode_log(contract, topics, data)
}

/**
 * Decode logs in batches
 */
pub fn decode_batch_logs(abi_url: String, tx_logs: Vec<TxLog>) -> Vec<EventLogParamResult> {
    let args = vec![format!(r#"{}"#, abi_url)];
    let response = curl_request_res(args).unwrap();
    let contract = Contract::load(response.as_bytes()).unwrap();

    let mut data_events: Vec<EventLogParamResult> = Vec::new();

    for tx_log in tx_logs {
        data_events.push(decode_log(contract.clone(), tx_log.topics, tx_log.data));
    }

    data_events
}

/**
 * Decode logs from topics and data
 */
fn decode_log(contract: Contract, topics: Vec<String>, data: String) -> EventLogParamResult {
    let mut logs_h256: Vec<H256> = Vec::new();

    for topic in topics.clone() {
        logs_h256.push(H256::from_str(&topic).unwrap())
    }

    let mut logs: Vec<DataLogParam> = Vec::new();

    let event_name = logs_h256.clone()[0];

    for (_, event) in contract.events {
        if event_name == event[0].signature() {
            let raw_log = RawLog {
                topics: logs_h256.clone(),
                data: hex::decode(&data[2..]).unwrap(),
            };

            let log = event[0].parse_log(raw_log).unwrap();

            for token in log.params {
                match token.value.clone() {
                    Token::Uint(value) => logs.push(DataLogParam {
                        name: token.name.clone(),
                        kind: "uint".to_string(),
                        value: value.to_string(),
                    }),
                    Token::Address(address) => logs.push(DataLogParam {
                        name: token.name.clone(),
                        kind: "address".to_string(),
                        value: format!("0x{}", hex::encode(address).to_string()),
                    }),
                    Token::Int(value) => logs.push(DataLogParam {
                        name: token.name.clone(),
                        kind: "int".to_string(),
                        value: value.to_string(),
                    }),
                    Token::Bool(value) => logs.push(DataLogParam {
                        name: token.name.clone(),
                        kind: "bool".to_string(),
                        value: value.to_string(),
                    }),
                    Token::Bytes(value) => logs.push(DataLogParam {
                        name: token.name.clone(),
                        kind: "bytes".to_string(),
                        value: hex::encode(value).to_string(),
                    }),
                    Token::String(value) => logs.push(DataLogParam {
                        name: token.name.clone(),
                        kind: "string".to_string(),
                        value: value.to_string(),
                    }),
                    _ => {
                        log::info!("Other token: {:?}", token.value.clone());
                    }
                }
            }

            return EventLogParamResult {
                event_name: event[0].clone().name,
                params: logs,
                success: true,
                error_msg: "".to_string(),
            };
        }
    }

    return EventLogParamResult {
        event_name: "".to_string(),
        params: Vec::new(),
        success: false,
        error_msg: "".to_string(),
    };
}

#[marine]
pub fn decode_input_to_get_method_name(abi_url: String, input: String) -> String {
    let args = vec![format!(r#"{}"#, abi_url)];
    let response = curl_request_res(args).unwrap();
    let contract = Contract::load(response.as_bytes()).unwrap();

    let input_bytes = hex::decode(&input[2..]).unwrap();

    for (name, function) in contract.functions {
        if &input_bytes[0..4] == function[0].short_signature() {
            return name;
        }
    }

    return "".to_string();
}
