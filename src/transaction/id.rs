use hex::{ ToHex, FromHex };
use std::convert::AsRef;
use serde::de::{ Deserialize, Deserializer, self, Visitor };
use serde::ser::{ Serialize, Serializer };
use std::fmt;
use std::marker::PhantomData;
use std::mem::size_of;

use crate::protocol::{ ToBuffer, FromBuffer, BufferSize, BufMut, Buf };

pub trait Txid = ToHex + FromHex + Default + AsRef<[u8]> + AsMut<[u8]>;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct NtId<Id: Txid>(Id);

impl<Id: Txid> NtId<Id> {
    pub fn new(id: Id) -> Self {
        NtId::<Id>(id)
    }
}

impl<Id: Txid> ToBuffer for NtId<Id> {
    fn buffer_dump(&self, buffer: &mut impl BufMut) {
        buffer.put_slice(self.0.as_ref());
    }
}

impl<Id: Txid> BufferSize for NtId<Id> {
    fn buffer_size(&self) -> usize {
        size_of::<Id>()
    }
}

impl<Id: Txid> FromBuffer for NtId<Id> {
    fn parse_buffer(bytes: &mut impl Buf) -> Self {
        let mut id = Id::default();
        bytes.copy_to_slice(id.as_mut());
        NtId::new(id)
    }
}

impl<Id: Txid> Serialize for NtId<Id> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = self.0.encode_hex_upper::<String>();
        serializer.serialize_str(&s.as_str())
    }
}

struct HexVisitor<Id: Txid>{
    marker: PhantomData<Id>,
}

impl<Id: Txid> HexVisitor<Id> {
    fn new() -> Self {
        HexVisitor {
            marker: PhantomData::<Id>
        }
    }
}

impl<'de, Id: Txid> Visitor<'de> for HexVisitor<Id> {
    type Value = NtId<Id>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an bytes hex express")
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: de::Error, {
        self.visit_str(v.as_str())
    }

    fn visit_borrowed_str<E>(self, v: &'de str) -> Result<Self::Value, E> where E: de::Error, {
        self.visit_str(v)
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: de::Error, {
        match Id::from_hex(v) {
            Ok(r) => Ok(NtId::<Id>(r)),
            Err(_) => {
                Err(de::Error::invalid_value(de::Unexpected::Str(&v), &self))
            },
        }
    }
}

impl<'de, Id: Txid> Deserialize<'de> for NtId<Id> {
    fn deserialize<D>(deserializer: D) -> Result<NtId<Id>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HexVisitor::<Id>::new())
    }
}

#[cfg(test)]
mod tests {
    use super::NtId;
    use serde::Serialize;
    use serde::Deserialize;
    use bytes::BytesMut;
    use crate::protocol::{ FromBuffer, ToBuffer, BufferSize, Buf };

    #[derive(Serialize,Deserialize, Debug)]
    struct Test {
        test: NtId<Vec<u8>>,
    }

    impl Test {
        pub fn new () -> Self {
            let v = vec![1,2,3,4];
            Test {
                test: NtId(v),
            }
        }
    }

    #[test]
    fn test_json() -> serde_json::Result<()> {
        let t = Test::new();
        let s = serde_json::to_string(&t)?;
        println!("test rs: {}", s);
        let r: Test = serde_json::from_str(s.as_str())?;
        println!("{:?}", r);
        Ok(())
    }

    #[test]
    fn test_protocol() {
        let v = [3u8;32];
        let sss = NtId::new(v);
        let mut buffer = BytesMut::with_capacity(sss.buffer_size());
        sss.buffer_dump(&mut buffer);
        println!("encoded {:?}", buffer);
        let mut sbuffer = buffer.to_bytes();
        let ssss = NtId::<[u8;32]>::parse_buffer(&mut sbuffer);
        assert_eq!(sss, ssss);
    }
}

