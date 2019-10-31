use serde::{Serialize, Deserialize};
use crate::utils::Hasher;
use super::tx_input::TxInput;
use super::tx_output::TxOutput;
use super::validater::Validater;
use crate::storage::Storage;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell<H: Hasher + Default, V: Validater + Default>{
    txid: H::Output,
    inputs: Vec<TxInput<H>>,
    outputs: Vec<TxOutput<H, V>>,
}

impl <H: Hasher + Default, V: Validater + Default> Cell<H, V> {
    pub fn init<S>(&mut self, data: &Vec<u8>, storage: S) where S: Storage {
        let hasher = H::new();
        hasher.write(data);
        self.txid = hasher.finish();

        for output in &mut self.outputs {
            output.init(&storage);
        }
    }
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};
    
    #[derive(Serialize, Deserialize, Debug)]
    struct HasherImpl {}

    impl super::Hasher for HasherImpl {
        type Output = u64;

        fn new() -> Self {
            HasherImpl {}
        }

        fn write(&self, _data: &Vec<u8>) {

        }

        fn finish(self) -> Self::Output {
            0
        }
    }

    #[test]
    fn test_serde() {
        // let inputs = Vec::<super::TxInput<HasherImpl>>::new();
        // let outputs = Vec::<super::TxOutput<HasherImpl>>::new();
        // let r = super::Cell::<HasherImpl> {txid: 0,inputs, outputs};
        // let j = serde_json::to_string(&r).unwrap();
        // println!("{}", j);
    }
}

