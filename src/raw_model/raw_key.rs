use super::{varint::VarInt, BufferView, ParseError};

#[derive(Debug)]
pub struct RawKey {
    key_length: VarInt,
    pub key_bytes: Vec<u8>,
}

impl BufferView for RawKey {
    fn size(&self) -> usize {
        self.key_length.size() + self.key_bytes.len()
    }

    fn from_buffer(buffer: &Vec<u8>, offset: usize) -> Result<Self, super::ParseError> {
        let mut rel_offset = offset;
        let key_length = VarInt::from_buffer(&buffer, rel_offset)?;
        rel_offset += key_length.size();

        let key_length_value = match key_length.value() {
            Ok(value) => value as usize,
            Err(e) => {
                return Err(ParseError {
                    offset,
                    rel_offset,
                    reason: e,
                })
            }
        };

        if buffer.len() < rel_offset + key_length_value {
            return Err(ParseError {
                offset,
                rel_offset,
                reason: "Unexpected end of buffer: key length overflows buffer".to_string(),
            });
        }

        let key_bytes = buffer[rel_offset..(rel_offset + key_length_value)].to_vec();

        return Ok(RawKey {
            key_length,
            key_bytes,
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_model::{raw_key::RawKey, BufferView};

    #[test]
    fn simple_string() {
        let raw_key = RawKey::from_buffer(&vec![0, 0, 133, 72, 101, 108, 108, 111], 2).unwrap();
        assert_eq!(6, raw_key.size());
        assert_eq!(1, raw_key.key_length.size());
        assert_eq!(5, raw_key.key_length.value().unwrap());
        assert_eq!(vec![72, 101, 108, 108, 111], raw_key.key_bytes);
    }

    #[test]
    fn too_big_string() {
        let raw_key = RawKey::from_buffer(&vec![135, 21, 10], 0);
        assert!(raw_key.is_err_and(|x| { x.offset == 0 && x.rel_offset == 1 }));
    }

    #[test]
    fn string_size_too_long() {
        let raw_key = RawKey::from_buffer(&vec![135, 2, 3, 1], 1);
        assert!(raw_key.is_err_and(|x| { x.offset == 1 && x.rel_offset == 2 }));
    }
}
