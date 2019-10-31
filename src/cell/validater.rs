pub trait Validater {

    fn is_ready(&self) -> bool;

    fn load(&self, code: &Vec<u8>) -> bool;

    fn validate(&self, data: &Vec<u8>) -> bool;
}
