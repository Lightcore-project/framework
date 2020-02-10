use super::id::{ Txid, NtId };
use serde::{ Serialize, Deserialize };
use std::convert::{ TryInto };
use crate::protocol::{ BufferSize, ToBuffer, FromBuffer, BufMut, Buf };

#[derive(Serialize, Deserialize, Debug)]
pub struct Script<Id: Txid> {
    pub txid: NtId<Id>,
    pub n: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output <Id: Txid> {
    pub capacity: u64,
    pub data: Vec<u8>,
    pub script: Option<Script<Id>>,
}

impl<Id: Txid> ToBuffer for Output<Id> {
    fn buffer_dump(&self, buffer: &mut impl BufMut) {
        buffer.put_u64(self.capacity);
        buffer.put_slice(self.data.as_ref());
    }
}

impl<Id: Txid> BufferSize for Output<Id> {
    fn buffer_size(&self) -> usize {
        let mut txid_len: u64 = 0;
        if let Some(script) = &self.script {
            txid_len = script.txid.buffer_size().try_into().unwrap();
        }
        let size = self.capacity + 8 + txid_len;
        size.try_into().unwrap()
    }
}

impl<Id: Txid> FromBuffer for Output<Id> {
    fn parse_buffer(bytes: &mut impl Buf) -> Self {
        let txid = NtId::parse_buffer(bytes);
        let n = bytes.get_u64();
        let capacity = bytes.get_u64();
        let mut data = Vec::with_capacity(capacity.try_into().unwrap());
        bytes.copy_to_slice(data.as_mut());
        Output {
            capacity,
            data,
            script: Some(Script {
                txid,
                n
            })
        }
    }
}

impl<Id: Txid> Output<Id> {
    pub fn new(capacity: u64, data: Vec<u8>, script: Option<Script<Id>>) -> Self {
        Output {
            capacity, data, script,
        }
    }
}

