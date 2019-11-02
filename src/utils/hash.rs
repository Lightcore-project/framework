use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use hex::FromHex;
use hex::ToHex;
use std::cmp::PartialEq;

pub trait DeserializeDe: for <'de> Deserialize<'de> {}
impl<T: ?Sized> DeserializeDe for T where T: for<'de> Deserialize<'de> {}

pub trait Hasher {
    type Output: Debug + Serialize + DeserializeDe + ToHex + FromHex + PartialEq;

    fn new() -> Self;
    
    fn write(&self, data: &Vec<u8>);

    fn finish(self) -> Self::Output;
}

