use serde::{ Serialize, Deserialize };
use std::convert::TryInto;

use super::id::{ Txid, NtId };
use crate::protocol::{ FromBuffer, ToBuffer, BufferSize, Buf, BufMut };

#[derive(Serialize, Deserialize, Debug)]
pub struct Input <Id: Txid> {
    pub txid: NtId<Id>,
    pub n: u64,
    pub capacity: u64,
    pub args: Vec<u8>,
}

impl<Id: Txid> Input<Id> {
    pub fn new(txid: Id, n: u64, capacity: u64, args: Vec<u8>) -> Self {
        Input {
            txid: NtId::new(txid),
            n,
            capacity,
            args
        }
    }
}

impl<Id: Txid> ToBuffer for Input<Id> {
    fn buffer_dump(&self, buffer: &mut impl BufMut) {
        self.txid.buffer_dump(buffer);
        buffer.put_u64(self.n);
        buffer.put_u64(self.capacity);
        buffer.put_slice(self.args.as_ref());
    }
}

impl<Id: Txid> BufferSize for Input<Id> {
    fn buffer_size(&self) -> usize {
        let txid_len: u64 = self.txid.buffer_size().try_into().unwrap();
        let r = txid_len + 8 + self.capacity;
        r.try_into().unwrap()
    }
}

impl<Id: Txid> FromBuffer for Input<Id> {
    fn parse_buffer(bytes: &mut impl Buf) -> Self {
        let txid = NtId::parse_buffer(bytes);
        let n = bytes.get_u64();
        let capacity = bytes.get_u64();
        let mut args = Vec::with_capacity(capacity.try_into().unwrap());
        bytes.copy_to_slice(args.as_mut());
        Input {
            txid,
            n,
            capacity,
            args
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Input;
    use crate::protocol::{ BufferSize };

    #[test]
    fn test_all() {
        let txid = [0u8;32];
        let input = Input::new(txid, 0, 0, Vec::new());
        let length = input.buffer_size();
        println!("L: {}", length);

    }

}

