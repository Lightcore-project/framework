use crate::block::BlockHeader;
use crate::cell::Cell;
use crate::utils::Hasher;
use crate::network::Network;

pub trait Consensus<T,H: Hasher + Default, N: Network> {
    fn propose (&self, txs: Vec<Cell<H>>);
    
    fn commit (&self, handler: impl FnOnce(BlockHeader<T,H,N>));
}
