pub mod nib;
pub mod raw_class;
pub mod raw_key;
pub mod raw_object;
pub mod raw_value;
pub mod varint;

#[derive(Debug)]
pub struct ParseError {
    offset: usize,
    rel_offset: usize,
    reason: String,
}

pub trait BufferView {
    fn size(&self) -> usize;
    fn from_buffer(buffer: &Vec<u8>, offset: usize) -> Result<Self, ParseError>
    where
        Self: Sized;
}
