pub trait Storage {

    fn set(&self,field: String, id: String, data: Vec<u8>);

    fn get(&self,field: String, id: String) -> Option<Vec<u8>>;
}
