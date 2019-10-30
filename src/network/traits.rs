pub trait NetworkInterface {
    type NodeId;
    
    fn send(&self, id: Self::NodeId, data: Vec<u8>);

    fn recv(&self, handler: impl FnOnce(Self::NodeId, Vec<u8>));
}
