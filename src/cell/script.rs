use serde::{Serialize, Deserialize};
use crate::utils::Hasher;
use super::validater::Validater;
use crate::storage::Storage;
use hex::ToHex;

#[derive(Serialize, Deserialize, Debug)]
pub struct Script <H: Hasher, V: Validater> {
    pub txid: H::Output,
    pub index: u64,

    #[serde(skip)]
    pub validater: V,
}

impl<H: Hasher, V: Validater> Script<H,V> {
    pub fn init<S>(&mut self, storage: &S) -> bool where S:Storage {
        let field_key = String::from("script-code");
        let hex_txid = self.txid.encode_hex::<String>();
        let id_key = format!("{}", format_args!("code-{}-{}",hex_txid , self.index));
        let result = storage.get(field_key, id_key);
        match result {
            Some(code) => {
                let success = self.validater.load(&code);
                success && self.validater.is_ready()
            },
            None => {
                false
            }
        }
    }
}



