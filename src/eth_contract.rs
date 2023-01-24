use std::str::FromStr;

use crate::{curl_request_res, eth_calls::eth_call, types::TxCall};
use ethabi::{Contract, Token};
use ethereum_types::{H160, U256};
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
