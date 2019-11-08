pub trait Grouping {
    type NodeID;

    fn group(index: u128, input: Option<Self::NodeID>) -> Vec<Self::NodeID>;
}
