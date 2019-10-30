pub trait HashInterface {
    type Output;

    fn new() -> Self;
    
    fn update(&self, data: &Vec<u8>, size: u64);

    fn digest(self) -> Self::Output;
}

