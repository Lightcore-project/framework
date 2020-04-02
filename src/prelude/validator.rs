use crate::Result;
use async_trait::async_trait;
use core::convert::AsRef;

pub trait Validator {
    type ExecuteResult: AsRef<[u8]>;

    /// Load code into validator;
    fn load(&self, code: impl AsRef<[u8]>) -> Result<()>;

    /// Execute code with args, return result after execute;
    fn executor(&self, args: impl AsRef<[u8]>) -> Result<Self::ExecuteResult>;
}
