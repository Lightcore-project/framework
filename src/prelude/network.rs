use async_trait::async_trait;
use core::convert::AsRef;

/// the trait for network
#[async_trait]
pub trait Network {
    type NodeId;

    type Error;

    type RecvByteType: AsRef<[u8]>;

    async fn send(&self, id: Self::NodeId, data: impl AsRef<[u8]>) -> Result<(), Self::Error>;

    async fn recv(&self) -> Result<(Self::NodeId, Self::RecvByteType), Self::Error>;
}
