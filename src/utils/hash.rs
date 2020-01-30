use std::fmt::Debug;
use serde::{Serialize, Deserialize};
use hex::FromHex;
use hex::ToHex;
use std::cmp::PartialEq;
use std::convert::AsRef;

pub trait DeserializeDe: for <'de> Deserialize<'de> {}
impl<T: ?Sized> DeserializeDe for T where T: for<'de> Deserialize<'de> {}

pub trait Hasher: Default {
    type Output: Debug + Serialize + DeserializeDe + ToHex + FromHex + PartialEq;

    fn write(&mut self, data: impl AsRef<[u8]>);

    fn finish(self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use tiny_keccak::Sha3;
    use tiny_keccak::Hasher;

    pub struct SHA3 {
        state: Sha3,
    }

    impl Default for SHA3 {
        fn default() -> Self {
            SHA3 { state: Sha3::v256() }
        }
    }

    impl super::Hasher for SHA3 {
        type Output = [u8; 32];

        fn write(&mut self, data: impl AsRef<[u8]>) {
            self.state.update(data.as_ref());
        }

        fn finish(self) -> Self::Output {
            let mut output = [0; 32];
            self.state.finalize(&mut output);
            output
        }
    }

    #[test]
    fn test() {
        let state = SHA3::default();

    }
}

