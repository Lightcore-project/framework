use std::convert::AsRef;

/// the trait for storage.
pub trait Storage {
    type Error;

    fn field<F>(&mut self, field: F)
        where F: AsRef<[u8]>;

    fn set<K>(&mut self, key: K, value: Vec<u8>) -> Result<Option<Vec<u8>>, Self::Error>
        where K: AsRef<[u8]>;

    fn get<K>(&mut self, key: K) -> Result<Option<Vec<u8>>, Self::Error>
        where K: AsRef<[u8]>;

    fn del<K>(&mut self, key: K) -> Result<Option<Vec<u8>>, Self::Error>
        where K: AsRef<[u8]>;
}

#[cfg(test)]
mod tests {
    use sled::Db;
    use super::Storage;
    use hex::ToHex;
    use std::string::String;

    struct SledStorage {
        db: Db,
    }

    impl SledStorage {
        pub fn new() -> Self {
            let db = sled::open("test/test.db").unwrap();
            SledStorage { db }
        }
    }

    impl Storage for SledStorage {
        type Error = sled::Error;

        fn field<F>(&mut self, field: F) where F: AsRef<[u8]> {
            println!("{}", field.encode_hex::<String>());
        }

        fn set<K>(&mut self, key: K, value: Vec<u8>) -> Result<Option<Vec<u8>>, Self::Error> where K: AsRef<[u8]> {
            let r = self.db.insert(key, value)?;
            match r {
                Some(_value) => {
                    Ok(Some(Vec::from(_value.as_ref())))
                },
                None => Ok(None),
            }
        }

        fn get<K>(&mut self, key: K) -> Result<Option<Vec<u8>>, Self::Error> where K: AsRef<[u8]> {
            let r = self.db.get(key)?;
            match r {
                Some(_value) => {
                    Ok(Some(Vec::from(_value.as_ref())))
                },
                None => Ok(None),
            }
        }


        fn del<K>(&mut self, key: K) -> Result<Option<Vec<u8>>, Self::Error> where K: AsRef<[u8]> {
            let r = self.db.remove(key)?;
            match r {
                Some(_value) => {
                    Ok(Some(Vec::from(_value.as_ref())))
                },
                None => Ok(None),
            }
        }
    }

    #[test]
    fn test() -> Result<(), sled::Error> {
        let mut db = SledStorage::new();
        db.field(b"hello");
        db.set(b"key", vec![0,1,2,3,4,5,6,7])?;
        let value = db.get(b"key")?;
        println!("{:?}", value);
        Ok(())
    }

}

