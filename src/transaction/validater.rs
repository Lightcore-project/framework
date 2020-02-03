use std::collections::HashMap;
use crate::utils::Hasher;
use hex::ToHex;
use crate::storage::Storage;

pub trait Validater: Default {

    fn is_ready(&self) -> bool;

    fn load(&self, code: &Vec<u8>) -> bool;

    fn validate(&self, data: &Vec<u8>) -> bool;
}

pub struct ValidaterExecutor<V: Validater> {
    validaters: HashMap<String, V>,
}

impl<V: Validater> ValidaterExecutor<V> {
    pub fn new() -> Self {
        let validaters = HashMap::new();
        ValidaterExecutor {
            validaters,
        }
    }

    pub fn validate<H>(&mut self, txid: &H::Output, index: u64, code: &Vec<u8>, data: &Vec<u8>) 
        where H: Hasher,
    {
        let hex_txid = txid.encode_hex::<String>();
        let key = format!("{}", format_args!("code-{}-{}", hex_txid, index));
        let checkers = &self.validaters;
        match checkers.get(&key) {
            Some(validater) => validater.validate(data)
        }
        
    }

    pub fn _validate<H, S>(&mut self, txid: &H::Output, index: u64, data: &Vec<u8>, storage: &S) -> bool where H: Hasher, S: Storage {
        let hex_txid = txid.encode_hex::<String>();
        let key = format!("{}", format_args!("code-{}-{}", hex_txid, index));

        let validater_checker = &self.validaters;
        match validater_checker.get(&key) {
            Some(validater) => {
                // validate data
                validater.validate(data)
            },
            None => {
                // create validate and insert value
                let validater = V::default();
                let field_key = String::from("state");
                match storage.get(field_key, key.clone()) {
                    Some(code) => {
                        let result = validater.validate(&code);
                        self.validaters.insert(key, validater);
                        result
                    },
                    None => {
                        false
                    }
                }
            }
        }
    }
}

