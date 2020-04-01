use super::in_out::{Input, Output};
use crate::prelude::HashValue;
use crate::Error;

pub struct Transaction<Id: HashValue> {
    pub version: u64,
    pub txid: Id,
    pub n_inputs: u64,
    pub n_outputs: u64,
    pub inputs: Vec<Input<Id>>,
    pub outputs: Vec<Output<Id>>,
}

impl<Id: HashValue> Transaction<Id> {
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

#[cfg(test)]
mod tests {}
