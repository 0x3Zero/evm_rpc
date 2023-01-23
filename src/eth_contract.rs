use std::str::FromStr;

use crate::{curl_request_res, eth_calls::eth_call, types::TxCall};
use ethabi::Contract;
use ethereum_types::H160;
use marine_rs_sdk::marine;

#[marine]
pub fn contract_get_remote_abi(
    node_url: String,
    abi_url: String,
    method_name: String,
    contract_address: String,
) {
    let args = vec![format!(r#"{}"#, abi_url)];
    let response = curl_request_res(args).unwrap();
    let contract = Contract::load(response.as_bytes()).unwrap();
    let func = contract.function(&method_name).unwrap();
    let data_in_bytes = func.encode_input(&[]).unwrap();

    let params = TxCall {
        to: Some(H160::from_str(&contract_address).unwrap()),
        data: Some(data_in_bytes.into()),
        ..Default::default()
    };

    let response = eth_call(node_url, params, "latest".into()).result;
    // remove 0x in the response
    let supply_in_hex = response.as_str().strip_prefix("0x").unwrap_or(&response);
    log::info!("{:?}", i128::from_str_radix(&supply_in_hex, 16));
}
