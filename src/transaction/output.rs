use super::id::{ Txid, NtId };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Deserialize, Debug)]
pub struct Script<Id: Txid> {
    txid: NtId<Id>,
    n: u64,
}

impl<Id: Txid> Script<Id> {
    pub fn new(txid: Id, n: u64) -> Self {
        Script {
            txid: NtId::new(txid),
            n
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output <Id: Txid> {
    capacity: u64,
    data: Vec<u8>,
    script: Option<Script<Id>>,
}

impl<Id: Txid> Output<Id> {
    pub fn new(capacity: u64, data: Vec<u8>, script: Option<Script<Id>>) -> Self {
        Output {
            capacity, data, script,
        }
    }
}

