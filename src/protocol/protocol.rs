pub use bytes::{ BufMut, Buf };

pub trait ToBuffer {
    fn buffer_dump(&self, buffer: &mut impl BufMut);
}

pub trait BufferSize {
    fn buffer_size(&self) -> usize;
}

pub trait FromBuffer: Sized {
    fn parse_buffer(bytes: &mut impl Buf) -> Self;

    // fn parse_size(bytes: &mut impl Buf) -> usize;
}


