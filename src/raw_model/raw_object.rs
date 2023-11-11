use super::{varint::VarInt, BufferView};

#[derive(Debug)]
pub struct RawObject {
    pub class_index: VarInt,
    pub value_index: VarInt,
    pub value_count: VarInt,
}

impl BufferView for RawObject {
    fn size(&self) -> usize {
        self.class_index.size() + self.value_index.size() + self.value_count.size()
    }

    fn from_buffer(buffer: &Vec<u8>, offset: usize) -> Result<Self, super::ParseError> {
        let mut rel_offset = offset;

        let class_index = VarInt::from_buffer(&buffer, rel_offset)?;
        rel_offset += class_index.size();

        let value_index = VarInt::from_buffer(&buffer, rel_offset)?;
        rel_offset += value_index.size();

        let value_count = VarInt::from_buffer(&buffer, rel_offset)?;

        return Ok(RawObject {
            class_index,
            value_index,
            value_count,
        });
    }
}
