use serde::{Serialize, Deserialize};
use super::script::Script;
use crate::utils::Hasher;
use super::validater::Validater;
use crate::storage::Storage;

#[derive(Serialize, Deserialize, Debug)]
pub struct TxOutput <H: Hasher + Default, V: Validater + Default> {
    size: u64,
    data: Vec<u8>,
    script: Option<Script<H, V>>
}

impl<H: Hasher + Default, V: Validater + Default> TxOutput<H,V> {
    pub fn init<S>(&mut self, storage: &S) -> bool where S:Storage {
        match &mut self.script {
            Some(script) => {
                script.init(storage)
            },
            None => {
                true
            }
        }
    }
}

