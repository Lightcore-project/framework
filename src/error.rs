use core::convert::From;
use protobuf::ProtobufError;

pub enum LightcoreError {
    BalanceError,
    ProtobufError(ProtobufError),
}

impl From<ProtobufError> for LightcoreError {
    fn from(e: ProtobufError) -> Self {
        LightcoreError::ProtobufError(e)
    }
}

pub type Result<T> = core::result::Result<T, LightcoreError>;

