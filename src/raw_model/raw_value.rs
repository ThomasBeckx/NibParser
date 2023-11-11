use super::{varint::VarInt, BufferView};

// 0: int8, 1 byte
// 1: int16 LE, 2 bytes
// 2: int32 LE, 4 bytes
// 3: int64 LE, 8 bytes
// 4: true
// 5: false
// 6: float, 4 bytes
// 7: double, 8 bytes
// 8: data, varint , number of bytes as specified in varint
// 9: nil
// 10: object reference, 4 bytes uint32 LE coding an offset into the list of objects
fn data_type_size(data_type: u8) -> Option<i8> {
    let size = match data_type {
        8 => -1,
        4 | 5 | 9 => 0,
        0 => 1,
        1 => 2,
        2 | 6 | 10 => 4,
        3 | 7 => 8,
        _ => return None,
    };
    Some(size)
}

pub struct RawValue {
    pub key_index: VarInt,
    pub value_type: u8,
    pub data: Vec<u8>,
    extra_size: usize,
}

impl BufferView for RawValue {
    fn size(&self) -> usize {
        self.key_index.size() + 1 + self.data.len() + self.extra_size
    }

    fn from_buffer(buffer: &Vec<u8>, offset: usize) -> Result<Self, super::ParseError>
    where
        Self: Sized,
    {
        let mut rel_offset = offset;

        let key_index = VarInt::from_buffer(&buffer, rel_offset)?;
        rel_offset += key_index.size();

        let value_type = buffer
            .get(rel_offset)
            .ok_or(super::ParseError {
                offset,
                rel_offset,
                reason: "Failed to retrieve value type".to_string(),
            })?
            .to_owned();

        rel_offset += 1;

        let value_size = data_type_size(value_type).ok_or(super::ParseError {
            offset: offset,
            rel_offset: rel_offset,
            reason: format!("Invalid data type {}", value_type),
        })?;

        let mut extra_size = 0;
        let data: Vec<u8> = if value_size < 0 {
            let var_value_size = VarInt::from_buffer(&buffer, rel_offset)?;
            extra_size = var_value_size.size();

            rel_offset += var_value_size.size();
            let var_value_size_value = match var_value_size.value() {
                Ok(int) => int as usize,
                Err(e) => {
                    return Err(super::ParseError {
                        offset,
                        rel_offset,
                        reason: e,
                    })
                }
            };

            if buffer.len() <= rel_offset + var_value_size_value {
                return Err(super::ParseError {
                    offset,
                    rel_offset,
                    reason: "Buffer overflow when collecting dynamic data".to_string(),
                });
            }
            buffer[rel_offset..rel_offset + var_value_size_value].to_vec()
        } else {
            if buffer.len() <= rel_offset + value_size as usize {
                return Err(super::ParseError {
                    offset,
                    rel_offset,
                    reason: "Buffer overflow when collecting dynamic data".to_string(),
                });
            }
            buffer[rel_offset..rel_offset + value_size as usize].to_vec()
        };

        return Ok(RawValue {
            key_index,
            value_type,
            data,
            extra_size,
        });
    }
}
