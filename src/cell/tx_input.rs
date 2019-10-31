use serde::{Serialize, Deserialize};
use crate::utils::Hasher;

#[derive(Serialize, Deserialize, Debug)]
pub struct TxInput<H: Hasher> {
    txid: H::Output,
    index: u64,
    args: Vec<u8>,
}

