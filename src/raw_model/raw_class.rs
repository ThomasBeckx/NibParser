use std::fmt::Debug;

use super::{varint::VarInt, BufferView};

pub struct RawClass {
    class_name_length: VarInt,
    pub extra_values_count: VarInt,
    pub extra_values: Vec<u8>,
    pub class_name: Vec<u8>,
}

impl Debug for RawClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RawClass")
            .field("class_name_length", &self.class_name_length)
            .field("extra_values_count", &self.extra_values_count)
            .field("extra_values", &self.extra_values)
            .field("class_name", &self.class_name)
            .field(
                "class_name_string",
                &String::from_utf8(self.class_name.to_owned()),
            )
            .finish()
    }
}

impl BufferView for RawClass {
    fn size(&self) -> usize {
        self.class_name_length.size()
            + self.extra_values_count.size()
            + self.extra_values.len()
            + self.class_name.len()
    }

    fn from_buffer(buffer: &Vec<u8>, offset: usize) -> Result<Self, super::ParseError> {
        let mut rel_offset = offset;
        let class_name_length = VarInt::from_buffer(&buffer, rel_offset)?;
        rel_offset += class_name_length.size();

        let extra_values_count = VarInt::from_buffer(&buffer, rel_offset)?;
        rel_offset += extra_values_count.size();

        let extra_values_count_value =
            extra_values_count
                .value()
                .map_err(|reason| super::ParseError {
                    offset,
                    rel_offset,
                    reason,
                })? as usize
                * 4;

        if buffer.len() < rel_offset + extra_values_count_value {
            return Err(super::ParseError {
                offset,
                rel_offset,
                reason: "Unexpected end of buffer: extra values array length overflows buffer"
                    .to_string(),
            });
        }

        let extra_values = buffer[rel_offset..rel_offset + extra_values_count_value].to_vec();
        rel_offset += extra_values.len();

        let class_name_length_value =
            class_name_length
                .value()
                .map_err(|reason| super::ParseError {
                    offset,
                    rel_offset,
                    reason,
                })? as usize;

        if buffer.len() < rel_offset + class_name_length_value {
            return Err(super::ParseError {
                offset,
                rel_offset,
                reason: "Unexpected end of buffer: class name length overflows buffer".to_string(),
            });
        }

        let class_name = buffer[rel_offset..rel_offset + class_name_length_value].to_vec();

        Ok(RawClass {
            class_name_length,
            extra_values_count,
            extra_values,
            class_name,
        })
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_class() {}
}
