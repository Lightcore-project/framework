use serde::{ Serialize, Deserialize };

use super::id::{ Txid, NtId };
use crate::protocol::{ FromBuffer, ToBuffer, Buf, BufMut };

#[derive(Serialize, Deserialize, Debug)]
pub struct Input <Id: Txid> {
    txid: NtId<Id>,
    n: u64,
    capacity: u64,
    args: Vec<u8>,
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

/* impl<Id: Txid> ToBuffer for Input<Id> { */
    // fn serialize_buffer(&self, buffer: &mut impl BufMut) {
    //     self.txid.serialize_buffer(buffer);
    //     buffer.put_u64(self.n);
    //     buffer.put_u64(self.capacity);
    //     buffer.put_slice(self.args.as_ref());
    // }
    //
    // fn buffer_size(&self) -> usize {
    //     self.txid.buffer_size() + 16 + self.args.len()
    // }
/* } */

/* impl<Id: Txid> protocol::Deserialize for Input<Id> { */
    // fn deserialize(bytes: &mut impl Buf) -> Self {
    //     let mut id = Id::default();
    //     bytes.copy_to_slice(id.as_mut());
    //     NtId::new(id)
    // }
    //
    // fn size(_bytes: &mut impl Buf) -> usize {
    //     size_of::<Id>()
    // }
/* } */


#[cfg(test)]
mod tests {
    use super::Input;

    #[test]
    fn test_all() -> serde_json::Result<()> {
        let txid = [0u8;32];
        let input = Input::new(txid, 0, 0, Vec::new());
        let s = serde_json::to_string(&input)?;
        println!("L: {}", s);

        // let ss = r#"{"txid":"FFFFFFFFFF","n":1,"capacity":1,"args": "FFFFFFFFFF"}"#;
        let r : Input<Vec<u8>> = serde_json::from_str(&s)?;
        println!("debug is : {:?}", r);
        Ok(())
    }

}

