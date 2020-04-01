use crate::prelude::HashValue;

pub struct Input<Id: HashValue> {
    pub txid: Id,
    pub n: u64,
    pub capacity: u64,
    pub args: Vec<u8>,
}

pub struct Script<Id: HashValue> {
    pub txid: Id,
    pub n: u64,
}

pub struct Output<Id: HashValue> {
    pub capacity: u64,
    pub data: Vec<u8>,
    pub script: Option<Script<Id>>,
}

