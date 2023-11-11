pub fn cast_to_u32(buffer: &[u8]) -> Result<u32, String> {
    match buffer.to_owned().try_into() {
        Ok(b) => Ok(u32::from_le_bytes(b)),
        Err(e) => Err(format!("Failed to parse to u32: {:?}", e)),
    }
}

pub fn cast_to_i32(buffer: &[u8]) -> Result<i32, String> {
    match buffer.to_owned().try_into() {
        Ok(b) => Ok(i32::from_le_bytes(b)),
        Err(e) => Err(format!("Failed to parse to i32: {:?}", e)),
    }
}
