use hex::FromHex;
use hex::ToHex;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;
use std::convert::AsRef;
use std::fmt::Debug;

pub trait DeserializeDe: for<'de> Deserialize<'de> {}
impl<T: ?Sized> DeserializeDe for T where T: for<'de> Deserialize<'de> {}

pub trait HashValue = Debug + Serialize + DeserializeDe + ToHex + FromHex + PartialEq;

pub trait Hasher {
    type Output: HashValue;

    fn update(&mut self, data: impl AsRef<[u8]>);

    fn finalize(self) -> Self::Output;
}

#[cfg(test)]
mod tests {
    use super::Hasher;
    use hex::ToHex;
    use tiny_keccak::Sha3;

    pub struct SHA3 {
        state: Sha3,
    }

    impl Default for SHA3 {
        fn default() -> Self {
            SHA3 {
                state: Sha3::v256(),
            }
        }
    }

    impl Hasher for SHA3 {
        type Output = [u8; 32];

        fn update(&mut self, data: impl AsRef<[u8]>) {
            use tiny_keccak::Hasher;
            self.state.update(data.as_ref());
        }

        fn finalize(self) -> Self::Output {
            use tiny_keccak::Hasher;
            let mut output = [0; 32];
            self.state.finalize(&mut output);
            output
        }
    }

    #[test]
    fn test() {
        let mut state = SHA3::default();
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        state.update(v);
        let output = state.finalize();
        assert_eq!(
            String::from("c0188232190e0427fc9cc78597221c76c799528660889bd6ce1f3563148ff84d"),
            output.encode_hex::<String>()
        )
    }
}
