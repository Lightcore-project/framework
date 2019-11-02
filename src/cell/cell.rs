use serde::{Serialize, Deserialize};
use crate::utils::Hasher;
use crate::storage::Storage;
use hex::ToHex;
use std::convert::TryInto;
use super::validater::Validater;
use super::validater::ValidaterExecutor;

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInput<H: Hasher + Default> {
    txid: H::Output,
    index: u64,
    args: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Script<H: Hasher>{
    txid: H::Output,
    index: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput<H: Hasher + Default> {
    capacity: u64,
    data: Vec<u8>,
    script: Option<Script<H>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell<H: Hasher + Default>{
    txid: Option<H::Output>,
    inputs: Vec<TxInput<H>>,
    outputs: Vec<TxOutput<H>>,
}

impl <H: Hasher + Default> Cell<H> {
    pub fn init(&mut self, data: &Vec<u8>) -> bool {
        match &self.txid {
            Some(txid) => {
                let hasher = H::new();
                hasher.write(data);
                *txid == hasher.finish()
            },
            None => {
                let hasher = H::new();
                hasher.write(data);
                self.txid = Some(hasher.finish());
                true
            }
        }
    }

    // Check cell's balance.
    pub fn balance<S>(&self, storage: &S) -> bool where S: Storage {
        let mut input_value = 0;
        let mut output_value = 0;
        for input in &self.inputs {
            // Use storage to get states capacity.
            let field_key = String::from("state-capacity");
            let hex_txid = input.txid.encode_hex::<String>();
            let id_key = format!("{}", format_args!("code-{}-{}",hex_txid , input.index));
            let result = storage.get(field_key, id_key);
            match result {
                Some(capacity) => {
                    input_value += u64::from_be_bytes(capacity.as_slice().try_into().unwrap())
                },
                None => {
                    return false;
                }
            }
            input_value += 0;
        }

        for output in &self.outputs {
            output_value += output.capacity;
        }
        input_value >= output_value
    }

    pub fn validate<V, S>(&self, validater: &mut ValidaterExecutor<V>, storage: &S) -> bool where V: Validater + Default, S:Storage {
        for input in &self.inputs {
            let result = validater.validate::<H,S>(&input.txid, input.index, &input.args, storage);
            if !result {
                return false;
            }
        }
        true
    }

    pub fn execute<S>(self, storage: &S) where S: Storage {
        let capacity_field_key = String::from("state-capacity");
        let state_field_key = String::from("state");
        for input in self.inputs {
            let hex_txid = input.txid.encode_hex::<String>();
            let id_key = format!("{}", format_args!("code-{}-{}",hex_txid , input.index));
            storage.del(capacity_field_key.clone(), id_key.clone());
            storage.del(state_field_key.clone(), id_key);
        }

        let hex_txid = self.txid.unwrap().encode_hex::<String>();
        for (index, output) in self.outputs.iter().enumerate() {
            let id_key = format!("{}", format_args!("code-{}-{}",hex_txid , index));
            let capacity_bytes = Vec::<u8>::from(&(output.capacity.to_be_bytes())[..]);
            storage.set(capacity_field_key.clone(), id_key.clone(), capacity_bytes);
            storage.set(state_field_key.clone(), id_key, output.data.clone())
        }
    }
}
