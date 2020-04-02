use super::in_out::{Input, Output};
use crate::error::FrameworkError;
use crate::prelude::Hasher;
use crate::Result;

pub struct Transaction<H: Hasher> {
    pub version: u64,
    pub txid: H::Output,
    pub n_inputs: u64,
    pub n_outputs: u64,
    pub inputs: Vec<Input<H>>,
    pub outputs: Vec<Output<H>>,
}

impl<H: Hasher> Transaction<H> {
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
            Err(FrameworkError::BalanceError)
        }
    }
}

#[cfg(test)]
mod tests {}
