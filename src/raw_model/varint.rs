use std::fmt::{Debug, Display};

use crate::cast::cast_to_u32;

use super::BufferView;

pub struct VarInt {
    bytes: Vec<u8>,
}

impl Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "var_int[{}]: '{}'", self.size(), self.value().unwrap())
    }
}

impl Debug for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("VarInt")
            .field("bytes", &self.bytes)
            .field("value", &self.value())
            .finish()
    }
}

impl BufferView for VarInt {
    fn size(&self) -> usize {
        self.bytes.len()
    }

    fn from_buffer(buffer: &Vec<u8>, offset: usize) -> Result<VarInt, super::ParseError> {
        let mut rel_offset = offset;
        let mut finished = false;
        let mut bytes: Vec<u8> = vec![];
        while let Some(current) = buffer.get(offset) {
            finished = current >> 7 == 1;
            bytes.push(*current);
            rel_offset += 1;

            if finished {
                break;
            }
        }

        if !finished {
            return Err(super::ParseError {
                offset,
                rel_offset,
                reason: "Unexpected end of buffer: end of VarInt not reached.".to_string(),
            });
        }

        Ok(VarInt { bytes })
    }
}

impl VarInt {
    fn decode_bytes(&self) -> Vec<u8> {
        let mut decoded_bytes: Vec<u8> = vec![{ 0 }];

        for (index, byte) in self.bytes.iter().enumerate() {
            let value = byte & 0x7F;
            let filled = ((index * 7) % 8) as u32;

            let fill_position = decoded_bytes.len() - 1;
            decoded_bytes[fill_position] =
                decoded_bytes[fill_position] | (value.checked_shl(filled).unwrap_or(0));
            if filled > 1 {
                decoded_bytes.push(value >> (7 - (8 - filled)));
            }
        }

        decoded_bytes
    }

    pub fn value(&self) -> Result<u32, String> {
        let mut bytes = self.decode_bytes();
        while bytes.len() < 4 {
            bytes.push(0);
        }
        cast_to_u32(&bytes)
    }
}

#[cfg(test)]
mod tests {
    use crate::raw_model::{varint::VarInt, BufferView};

    #[test]
    fn simple_onebyte_var() {
        let var_int = VarInt::from_buffer(&vec![146], 0).unwrap();
        assert_eq!(1, var_int.size());
        assert_eq!(18, var_int.value().unwrap());
    }

    #[test]
    fn simple_twobytes_var() {
        let var_int = VarInt::from_buffer(&vec![127, 129], 0).unwrap();
        assert_eq!(2, var_int.size());
        assert_eq!(255, var_int.value().unwrap());
    }

    #[test]
    fn invalid_var_int() {
        let var_int = VarInt::from_buffer(&vec![0, 130, 127, 125], 2);
        assert!(var_int.is_err_and(|x| { x.offset == 2 && x.rel_offset == 3 }));
    }
}
