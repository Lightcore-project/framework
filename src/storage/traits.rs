pub trait StorageInterface {
    fn set(&self, id: &Vec<u8>, data: Vec<u8>);

    fn get(&self, id: &Vec<u8>) -> Vec<u8>;
}
