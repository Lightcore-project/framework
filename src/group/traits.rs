pub trait GroupInterface {
    type NodeID;

    fn group(index: u128, input: Option<Self::NodeID>) -> Vec<NodeID>;
}