use crate::utils::Hasher;
use crate::network::Network;
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Serialize, Default)]
pub struct BlockHeader<T,H: Hasher, N: Network> {
    blockid: H::Output,
    timestamp: u64,
    height: u64,
    miner: N::NodeId,
    merkle: H::Output,
    extend: T,
}

