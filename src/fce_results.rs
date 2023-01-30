/*
 * Copyright 2021 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::jsonrpc_helpers::JSON_RPC;
use crate::types::{ResultSerde, Tx, TxSerde};
use marine_rs_sdk::marine;
use serde_json::Value;
pub type Result<T> = std::result::Result<T, T>;

// Result
#[marine]
#[derive(Debug)]
pub struct JsonRpcResult {
    pub jsonrpc: String,
    pub result: String,
    pub error: String,
    pub id: u64,
}

impl JsonRpcResult {
    pub fn from_res(raw_result: Result<String>) -> Self {
        let jsonrpc = JSON_RPC.into();
        match raw_result {
            Ok(res) => {
                let result_obj: Value = serde_json::from_str(&res).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();
                let result = serde_json::from_value(result_obj["result"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    result,
                    error: "".to_string(),
                }
            }
            Err(err) => {
                let result_obj: Value = serde_json::from_str(&err).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    result: "".to_string(),
                    error: err,
                }
            }
        }
    }
}

// Block
#[marine]
#[derive(Debug)]
pub struct JsonRpcBlockResult {
    pub jsonrpc: String,
    pub transactions: Vec<Tx>,
    pub error: String,
    pub id: u64,
}

impl JsonRpcBlockResult {
    pub fn from_res(raw_result: Result<String>) -> Self {
        let jsonrpc = JSON_RPC.into();
        match raw_result {
            Ok(res) => {
                let result_obj: Value = serde_json::from_str(&res).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();
                let result: ResultSerde =
                    serde_json::from_value(result_obj["result"].clone()).unwrap();

                let txs = result
                    .transactions
                    .iter()
                    .map(|serde| Tx::from(serde))
                    .collect();
                Self {
                    jsonrpc,
                    id,
                    transactions: txs,
                    error: "".to_string(),
                }
            }
            Err(err) => {
                let result_obj: Value = serde_json::from_str(&err).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    transactions: Vec::new(),
                    error: err,
                }
            }
        }
    }
}

// Transaction
#[marine]
#[derive(Debug)]
pub struct JsonRpcTransactionResult {
    pub jsonrpc: String,
    pub transaction: Tx,
    pub error: String,
    pub id: u64,
}

impl JsonRpcTransactionResult {
    pub fn from_res(raw_result: Result<String>) -> Self {
        let jsonrpc = JSON_RPC.into();
        match raw_result {
            Ok(res) => {
                let result_obj: Value = serde_json::from_str(&res).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();
                let result: TxSerde = serde_json::from_value(result_obj["result"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    transaction: Tx::from(&result),
                    error: "".to_string(),
                }
            }
            Err(err) => {
                let result_obj: Value = serde_json::from_str(&err).unwrap();
                let id: u64 = serde_json::from_value(result_obj["id"].clone()).unwrap();

                Self {
                    jsonrpc,
                    id,
                    transaction: Tx::default(),
                    error: err,
                }
            }
        }
    }
}

// Test
#[marine]
#[derive(Debug)]
pub struct TestResult {
    pub test_passed: bool,
    pub error: String,
}

impl From<Result<String>> for TestResult {
    fn from(result: Result<String>) -> Self {
        match result {
            Ok(res) => Self {
                test_passed: true,
                error: res,
            },
            Err(err) => Self {
                test_passed: false,
                error: err,
            },
        }
    }
}
