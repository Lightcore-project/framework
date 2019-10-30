use crate::utils::HashInterface;

pub struct Script <Hash: HashInterface> {
    pub capacity: u64,
    pub data: Vec<u8>,
    pub id: Hash::Output,
}

impl<Hash: HashInterface> Script<Hash> {
    pub fn new(data: Vec<u8>, capacity:u64) -> Self {
        let hasher = Hash::new();
        hasher.update(&data, capacity);
        let id = hasher.digest();
        Script {
            data,
            capacity,
            id,
        }
    }
}

