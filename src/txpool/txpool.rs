use std::collections::LinkedList;
use crate::cell::Cell;
use crate::utils::Hasher;
use crate::storage::Storage;

pub struct Txpool<'a, H: Hasher + Default, S: Storage> {
    // TODO: Use cache to optimize performance.
    txs: LinkedList<Cell<H>>,
    storage: &'a S,
}

impl<'a, H: Hasher + Default> Txpool<'a, H> {
    pub fn new() -> Self {
        let txs = LinkedList::new();
        Txpool {
            txs,
        }
    }

    // Receive txs from gossip network.
    // check cell's status and push cell in txpool.
    pub fn recv_txs_from_gossip(&self, txs: &Vec<Cell<H>>, validater: &mut ValidaterExecutor<V>) -> bool {
        let pre_txs = LinkedList::new();
        for tx in txs {
            if !tx.balance(self.storage) {
                false
            }
            if !tx.validate(validater, self.storage) {
                false
            }
        }
        true
    }

    // Receive txs fome block.
    // delete already has 
    pub fn recv_txs_from_block(&self, txs: &Vec<Cell<H>>) -> bool {

    }

    pub fn generate_block(&self, count: Option<u64>) -> Vec<Cell<H>> {

    }

    pub fn send_new_txs(&self, cell: Cell<H>) -> bool {

    }

}

