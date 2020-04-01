use async_trait::async_trait;
use std::convert::AsRef;

/// the trait for storage.
#[async_trait]
pub trait Storage {
    /// Error of storage low level impl.
    type Error;

    /// Value stored in low level
    type Value: AsRef<[u8]>;

    /// set field.
    async fn field<F>(&mut self, field: F)
    where
        F: AsRef<[u8]> + Send;

    /// set value
    async fn set<K, V>(&mut self, key: K, value: V) -> Result<Option<Self::Value>, Self::Error>
    where
        K: AsRef<[u8]> + Send,
        V: AsRef<[u8]> + Send;

    /// get value
    async fn get<K>(&self, key: K) -> Result<Option<Self::Value>, Self::Error>
    where
        K: AsRef<[u8]> + Send;

    /// delete value
    async fn del<K>(&mut self, key: K) -> Result<Option<Self::Value>, Self::Error>
    where
        K: AsRef<[u8]> + Send;
}

#[cfg(test)]
mod tests {
    use super::Storage;
    use async_trait::async_trait;
    use hex::ToHex;
    use sled::Db;
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

    #[async_trait]
    impl Storage for SledStorage {
        type Error = sled::Error;

        type Value = Vec<u8>;

        async fn field<F>(&mut self, field: F)
        where
            F: AsRef<[u8]> + Send,
        {
            let f = Vec::from(field.as_ref());
            println!("{}", f.encode_hex::<String>());
        }

        async fn set<K, V>(&mut self, key: K, value: V) -> Result<Option<Self::Value>, Self::Error>
        where
            K: AsRef<[u8]> + Send,
            V: AsRef<[u8]> + Send,
        {
            let v = Vec::from(value.as_ref());
            let r = self.db.insert(key, v)?;
            self.db.flush_async().await?;
            match r {
                Some(_value) => Ok(Some(Vec::from(_value.as_ref()))),
                None => Ok(None),
            }
        }

        async fn get<K>(&self, key: K) -> Result<Option<Self::Value>, Self::Error>
        where
            K: AsRef<[u8]> + Send,
        {
            self.db.flush_async().await?;
            let r = self.db.get(key)?;
            match r {
                Some(_value) => Ok(Some(Vec::from(_value.as_ref()))),
                None => Ok(None),
            }
        }

        async fn del<K>(&mut self, key: K) -> Result<Option<Self::Value>, Self::Error>
        where
            K: AsRef<[u8]> + Send,
        {
            let r = self.db.remove(key)?;
            self.db.flush_async().await?;
            match r {
                Some(_value) => Ok(Some(Vec::from(_value.as_ref()))),
                None => Ok(None),
            }
        }
    }

    #[async_std::test]
    async fn test() -> Result<(), sled::Error> {
        let mut db = SledStorage::new();
        db.field(b"hello").await;
        db.set(b"key", vec![0, 1, 2, 3, 4, 5, 6, 7]).await?;
        let value = db.get(b"key").await?;
        assert_eq!(value.unwrap(), vec![0, 1, 2, 3, 4, 5, 6, 7]);
        Ok(())
    }
}
