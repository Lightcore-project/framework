use serde::{ Serialize, Deserialize };
use std::convert::TryInto;

use super::id::{ Txid, NtId };
use super::input::Input;
use super::output::Output;
use crate::protocol::{ BufferSize, FromBuffer, ToBuffer, BufMut, Buf };

pub enum Error {
    BalanceError,
}

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

    pub fn balance(&self) -> Result<(), Error> {
        let mut input_len = 0;
        let mut output_len = 0;

        for input in &self.inputs {
            input_len += input.capacity;
        }

        for output in &self.outputs {
            output_len += output.capacity;
        }

        if input_len == output_len {
            Ok(())
        } else {
            Err(Error::BalanceError)
        }
    }
}

impl<Id: Txid> BufferSize for Transaction<Id> {
    fn buffer_size(&self) -> usize {
        let mut len = 8 + self.txid.buffer_size();
        for input in &self.inputs {
            len += input.buffer_size();
        }
        for output in &self.outputs {
            len += output.buffer_size();
        }
        len
    }
}

impl<Id: Txid> FromBuffer for Transaction<Id> {
    fn parse_buffer(bytes: &mut impl Buf) -> Self {
        let version = bytes.get_u64();
        let txid = NtId::parse_buffer(bytes);
        let ninputs = bytes.get_u64();
        let noutputs = bytes.get_u64();
        let mut inputs = Vec::new();
        let mut outputs = Vec::new();
        for _ in 0 .. ninputs {
            let input = Input::parse_buffer(bytes);
            inputs.push(input);
        }
        for _ in 0 .. noutputs {
            let output = Output::parse_buffer(bytes);
            outputs.push(output);
        }

        Transaction {
            version,
            txid,
            ninputs,
            noutputs,
            inputs,
            outputs
        }
    }
}

impl<Id: Txid> ToBuffer for Transaction<Id> {
    fn buffer_dump(&self, buffer: &mut impl BufMut) {
        buffer.put_u64(self.version);
        self.txid.buffer_dump(buffer);
        buffer.put_u64(self.ninputs);
        buffer.put_u64(self.noutputs);
        for input in &self.inputs {
            input.buffer_dump(buffer);
        }
        for output in &self.outputs {
            output.buffer_dump(buffer);
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

