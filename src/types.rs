use crate::bytes_type::Bytes;
use ethereum_types::{H160, H256, U256};
use marine_rs_sdk::marine;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::json;

fn null_to_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(d)?;
    let val = opt.unwrap_or_else(T::default);
    Ok(val)
}

#[marine]
#[derive(Debug, Default)]
pub struct Tx {
    pub block_hash: String,
    pub block_number: String,
    pub from: String,
    pub gas: String,
    pub gas_price: String,
    pub hash: String,
    pub input: String,
    pub nonce: String,
    pub to: String,
    pub transaction_index: String,
    pub value: String,
}

#[derive(Deserialize)]
struct GetTxResponse {
    #[serde(deserialize_with = "null_to_default")]
    pub result: Option<ResultSerde>,
}

#[derive(Debug, Default, Deserialize)]
pub struct ResultSerde {
    pub transactions: Vec<TxSerde>,
}

#[derive(Debug, Default, Deserialize)]
pub struct TxSerde {
    #[serde(rename = "blockHash")]
    pub block_hash: Option<String>,

    #[serde(rename = "blockNumber")]
    pub block_number: Option<String>,

    pub from: Option<String>,

    // // gas: QUANTITY - gas provided by the sender.
    pub gas: Option<String>,

    // // gasPrice: QUANTITY - gas price provided by the sender in Wei.
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<String>,

    // // hash: DATA, 32 Bytes - hash of the transaction.
    pub hash: Option<String>,

    // // input: DATA - the data send along with the transaction.
    pub input: Option<String>,

    // // nonce: QUANTITY - the number of transactions made by the sender prior to this one.
    pub nonce: Option<String>,

    // // to: DATA, 20 Bytes - address of the receiver. null when its a contract creation transaction.
    pub to: Option<String>,

    // // transactionIndex: QUANTITY - integer of the transactions index position in the block. null when its pending.
    #[serde(rename = "transactionIndex")]
    pub transaction_index: Option<String>,

    // // value: QUANTITY - value transferred in Wei.
    pub value: Option<String>,
}

impl From<&TxSerde> for Tx {
    fn from(ser: &TxSerde) -> Self {
        Self {
            block_hash: ser.block_hash.clone().unwrap_or_default(),
            block_number: ser.block_number.clone().unwrap_or_default(),
            from: ser.from.clone().unwrap_or_default(),
            gas: ser.gas.clone().unwrap_or_default(),
            gas_price: ser.gas_price.clone().unwrap_or_default(),
            hash: ser.hash.clone().unwrap_or_default(),
            input: ser.input.clone().unwrap_or_default(),
            nonce: ser.nonce.clone().unwrap_or_default(),
            to: ser.to.clone().unwrap_or_default(),
            transaction_index: ser.transaction_index.clone().unwrap_or_default(),
            value: ser.value.clone().unwrap_or_default(),
        }
    }
}

// ABI
#[marine]
#[derive(Debug, Deserialize)]
pub struct Abi {
    pub method: String,
}

#[derive(Default, Serialize)]
pub struct TxCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<H160>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<H160>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "gasPrice")]
    pub gas_price: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<U256>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Bytes>,
}
