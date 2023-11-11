use crate::{
    cast::{cast_to_i32, cast_to_u32},
    raw_model::{raw_class::RawClass, raw_key::RawKey, raw_object::RawObject, raw_value::RawValue},
};

pub struct Context {
    pub objects: Vec<RawObject>,
    pub keys: Vec<RawKey>,
    pub values: Vec<RawValue>,
    pub classes: Vec<RawClass>,
}

impl Context {
    pub fn parse(&self) -> Object {
        Object::from(self.objects.get(0).unwrap(), self)
    }
}

#[derive(Debug)]
pub struct Object {
    pub class: Class,
    pub values: Vec<Value>,
}

impl Object {
    pub fn from(raw: &RawObject, context: &Context) -> Object {
        let raw_class = match context
            .classes
            .get(raw.class_index.value().unwrap() as usize)
        {
            Some(class) => class,
            None => panic!(
                "Unable to retrieve class at index {}",
                raw.class_index.value().unwrap()
            ),
        };

        let start_value_index = raw.value_index.value().unwrap() as usize;
        let end_value_index = start_value_index + raw.value_count.value().unwrap() as usize;

        let values: Vec<Value> = (start_value_index..end_value_index)
            .map(|index| match context.values.get(index) {
                Some(raw_value) => Value::from(raw_value, context),
                None => panic!("Did not find enough values {}", index),
            })
            .collect();

        Object {
            class: Class::from(raw_class),
            values,
        }
    }
}

#[derive(Debug)]
pub struct Key {
    pub string: String,
}

impl Key {
    pub fn from(raw: &RawKey) -> Key {
        Key {
            string: String::from_utf8(raw.key_bytes.to_owned()).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct Value {
    pub key: Key,
    pub data: Data,
}

impl Value {
    pub fn get_data(raw: &RawValue, context: &Context) -> Data {
        match raw.value_type {
            0 => Data::Int8(i8::from_le_bytes(raw.data.to_owned().try_into().unwrap())),
            1 => Data::Int16(i16::from_le_bytes(raw.data.to_owned().try_into().unwrap())),
            2 => Data::Int32(i32::from_le_bytes(raw.data.to_owned().try_into().unwrap())),
            3 => Data::Int64(i64::from_le_bytes(raw.data.to_owned().try_into().unwrap())),
            4 => Data::Boolean(true),
            5 => Data::Boolean(false),
            6 => Data::Float(f32::from_le_bytes(raw.data.to_owned().try_into().unwrap())),
            7 => Data::Double(f64::from_le_bytes(raw.data.to_owned().try_into().unwrap())),
            8 => Data::Bytes(raw.data.to_owned()),
            9 => Data::Nil,
            10 => Data::Object(Object::from(
                context
                    .objects
                    .get(cast_to_u32(&raw.data).unwrap() as usize)
                    .unwrap(),
                context,
            )),
            _ => panic!("Invalid parsed value type {}", raw.value_type),
        }
    }

    pub fn from(raw: &RawValue, context: &Context) -> Value {
        let raw_key = match context.keys.get(raw.key_index.value().unwrap() as usize) {
            Some(key) => key,
            None => panic!("Unable to retrieve key at index {}", raw.key_index),
        };

        Value {
            key: Key::from(raw_key),
            data: Value::get_data(raw, context),
        }
    }
}

#[derive(Debug)]
pub struct Class {
    pub name: String,
    pub extra_values: Vec<i32>,
}

impl Class {
    pub fn from(raw: &RawClass) -> Class {
        let extra_values: Vec<i32> = (0..raw.extra_values.len() / 4)
            .map(|e| {
                cast_to_i32(&raw.extra_values[(e * 4) as usize..((e * 4) + 4) as usize]).unwrap()
            })
            .collect();
        Class {
            name: String::from_utf8(raw.class_name.to_owned()).unwrap(),
            extra_values,
        }
    }
}

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
#[derive(Debug)]
pub enum Data {
    Boolean(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float(f32),
    Double(f64),
    Bytes(Vec<u8>),
    Nil,
    Object(Object),
}
