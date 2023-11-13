use std::fmt::Display;

use crate::cast::cast_to_u32;

use super::{
    raw_class::RawClass, raw_key::RawKey, raw_object::RawObject, raw_value::RawValue, BufferView,
    ParseError,
};

#[derive(Debug)]
pub struct NibFile {
    version: String,
    object_count: u32,
    object_offset: u32,
    key_count: u32,
    key_offset: u32,
    value_count: u32,
    value_offset: u32,
    class_count: u32,
    class_offset: u32,
    buffer: Vec<u8>,
}

impl Display for NibFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "File {{\n  Buffer size: {}\n  Version: {},\n  Objects: [{}; {}],\n  Keys: [{}; {}],\n  Values: [{}; {}],\n  Classes: [{}; {}],\n}}", self.buffer.len(), self.version, self.object_offset, self.object_count, self.key_offset, self.key_count, self.value_offset, self.value_count, self.class_offset, self.class_count)
    }
}

impl NibFile {
    pub fn get_keys(&self) -> Result<Vec<RawKey>, ParseError> {
        let offset = self.key_offset as usize;
        let mut index = offset;
        let mut keys = vec![];

        while keys.len() < self.key_count as usize {
            if index >= self.value_offset as usize {
                return Err(ParseError {
                    offset: index,
                    rel_offset: index,
                    reason: "Keys memory overflowed into values".to_string(),
                });
            }
            match RawKey::from_buffer(&self.buffer, index) {
                Ok(key) => {
                    index += key.size();
                    keys.push(key);
                },
                Err(e) => {
                    println!(
                        "Partial key extraction, {} of {} retrieved: {:?}",
                        keys.len(),
                        self.key_count,
                        keys
                    );
                    return Err(e);
                }
            }
        }

        Ok(keys)
    }

    pub fn get_objects(&self) -> Result<Vec<RawObject>, ParseError> {
        let offset = self.object_offset as usize;
        let mut index = offset;
        let mut objects = vec![];

        while objects.len() < self.object_count as usize {
            if index >= self.key_offset as usize {
                return Err(ParseError {
                    offset: index,
                    rel_offset: index,
                    reason: "Objects memory overflowed into keys".to_string(),
                });
            }
            match RawObject::from_buffer(&self.buffer, index) {
                Ok(object) => {
                    index += object.size();
                    objects.push(object);
                },
                Err(e) => {
                    println!(
                        "Partial object extraction, {} of {} retrieved: {:?}",
                        objects.len(),
                        self.object_count,
                        objects
                    );
                    return Err(e);
                }
            }
        }

        Ok(objects)
    }

    pub fn get_values(&self) -> Result<Vec<RawValue>, ParseError> {
        let offset = self.value_offset as usize;
        let mut index = offset;
        let mut values = vec![];

        while values.len() < self.value_count as usize {
            if index >= self.class_offset as usize {
                return Err(ParseError {
                    offset: index,
                    rel_offset: index,
                    reason: "Values memory overflowed into classes".to_string(),
                });
            }
            match RawValue::from_buffer(&self.buffer, index) {
                Ok(value) => {
                    index += value.size();
                    values.push(value);
                },
                Err(e) => {
                    println!(
                        "Partial value extraction, {} of {} retrieved: {:?}",
                        values.len(),
                        self.value_count,
                        values
                    );
                    return Err(e);
                }
            }
        }

        Ok(values)
    }

    pub fn get_classes(&self) -> Result<Vec<RawClass>, ParseError> {
        let offset = self.class_offset as usize;
        let mut index = offset;
        let mut classes = vec![];

        while classes.len() < self.class_count as usize {
            if index >= self.buffer.len() as usize {
                return Err(ParseError {
                    offset: index,
                    rel_offset: index,
                    reason: "Classes memory overflowed out of buffer".to_string(),
                });
            }
            match RawClass::from_buffer(&self.buffer, index) {
                Ok(class) => {
                    index += class.size();
                    classes.push(class);
                }
                Err(e) => {
                    println!(
                        "Partial class extraction, {} of {} retrieved: {:?}",
                        classes.len(),
                        self.class_count,
                        classes
                    );
                    return Err(e);
                }
            }
        }

        Ok(classes)
    }

    pub fn from_buffer(buffer: Vec<u8>) -> Result<NibFile, ParseError> {
        let file_id = String::from_utf8(buffer[0..=9].to_vec());

        match file_id {
            Ok(id) => {
                if id != "NIBArchive" {
                    return Err(ParseError {
                        offset: 0,
                        rel_offset: 0,
                        reason: format!("Incorrect file identifier: {}", id),
                    });
                }
            }
            Err(e) => {
                return Err(ParseError {
                    offset: 0,
                    rel_offset: 0,
                    reason: format!("Invalid file identifier: {}", e),
                });
            }
        };

        let constant_one = cast_to_u32(&buffer[10..=13]).map_err(cast_error(10))?;
        let constant_two = cast_to_u32(&buffer[14..=17]).map_err(cast_error(14))?;
        let version: String = format!("{}.{}", constant_one, constant_two);

        let object_count = cast_to_u32(&buffer[18..=21]).map_err(cast_error(18))?;
        let object_offset = cast_to_u32(&buffer[22..=25]).map_err(cast_error(22))?;
        let key_count = cast_to_u32(&buffer[26..=29]).map_err(cast_error(26))?;
        let key_offset = cast_to_u32(&buffer[30..=33]).map_err(cast_error(30))?;
        let value_count = cast_to_u32(&buffer[34..=37]).map_err(cast_error(34))?;
        let value_offset = cast_to_u32(&buffer[38..=41]).map_err(cast_error(38))?;
        let class_count = cast_to_u32(&buffer[42..=45]).map_err(cast_error(42))?;
        let class_offset = cast_to_u32(&buffer[46..=49]).map_err(cast_error(46))?;

        // TODO: Add extra validation checks for sizes and offsets.

        Ok(NibFile {
            version,
            object_count,
            object_offset,
            key_count,
            key_offset,
            value_count,
            value_offset,
            class_count,
            class_offset,
            buffer,
        })
    }
}

fn cast_error(rel_offset: usize) -> impl Fn(String) -> ParseError {
    move |s| ParseError {
        offset: 0,
        rel_offset,
        reason: s,
    }
}
