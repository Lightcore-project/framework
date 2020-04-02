use crate::prelude::Hasher;

pub struct Input<H: Hasher> {
    pub txid: H::Output,
    pub n: u64,
    pub capacity: u64,
    pub args: Vec<u8>,
}

pub struct Script<H: Hasher> {
    pub txid: H::Output,
    pub n: u64,
}

pub struct Output<H: Hasher> {
    pub capacity: u64,
    pub data: Vec<u8>,
    pub script: Option<Script<H>>,
}

