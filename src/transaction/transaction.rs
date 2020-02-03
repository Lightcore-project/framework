use bytes::{ BufMut, BytesMut };
use serde::{ Serialize, Deserialize };
use std::convert::TryInto;

use super::id::{ Txid, NtId };
use super::input::Input;
use super::output::Output;

use crate::protocol;

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction<Id: Txid> {
    version: u64,
    txid: NtId<Id>,
    ninputs: u64,
    noutputs: u64,
    inputs: Vec<Input<Id>>,
    outputs: Vec<Output<Id>>,
}

impl<Id: Txid> Transaction<Id> {
    pub fn new(version: u64, txid: Id, inputs: Vec<Input<Id>>, outputs: Vec<Output<Id>>) -> Self {
        Transaction {
            version,
            txid: NtId::new(txid),
            ninputs: inputs.len().try_into().unwrap(),
            noutputs: outputs.len().try_into().unwrap(),
            inputs,
            outputs,
        }
    }
}

#[cfg(test)]
mod tests {
    struct Tese {
        test: Vec<u8>,
    }

    #[test]
    fn test_bytes() -> serde_json::Result<()> {
        Ok(())
    }
}

