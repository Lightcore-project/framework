use super::in_out::{Input, Output};
use crate::prelude::HashValue;
use crate::protocol::transaction::Transaction as ProtoTransaction;
use crate::{error::LightcoreError, Result};

pub struct Transaction<Id: HashValue> {
    pub version: u64,
    pub txid: Id,
    pub n_inputs: u64,
    pub n_outputs: u64,
    pub inputs: Vec<Input<Id>>,
    pub outputs: Vec<Output<Id>>,
}

impl<Id: HashValue> Transaction<Id> {
    pub fn from_proto(bytes: &[u8]) -> Result<Self> {
        let pt: ProtoTransaction = protobuf::parse_from_bytes(bytes)?;
        let t = Transaction {
            version: 0,
            txid: [0u8; 32],
            n_outputs: 0,
            n_inputs: 0,
            inputs: Vec::new(),
            outputs: Vec::new(),
        };
        Ok(t)
    }

    pub fn balance(&self) -> Result<()> {
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
            Err(LightcoreError::BalanceError)
        }
    }
}

#[cfg(test)]
mod tests {}
