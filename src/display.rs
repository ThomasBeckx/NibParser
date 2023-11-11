use crate::data::{Class, Data, Key, Object, Value};

pub trait JSON {
    fn to_json(&self) -> String;
}

impl JSON for Object {
    fn to_json(&self) -> String {
        format!(
            "{{ \"class\": {}, {} }}",
            self.class.to_json(),
            self.values
                .iter()
                .map(|v| { v.to_json() })
                .collect::<Vec<_>>()
                .join(",")
        )
    }
}

impl JSON for Key {
    fn to_json(&self) -> String {
        format!("{}", self.string)
    }
}

impl JSON for Value {
    fn to_json(&self) -> String {
        format!("\"{}\": {}", self.key.to_json(), self.data.to_json())
    }
}

impl JSON for Class {
    fn to_json(&self) -> String {
        if self.extra_values.len() == 0 {
            format!("\"{}\"", self.name)
        } else {
            format!(
                "{{ \"name\": \"{}\", \"extra_values\": {:?} }}",
                self.name, self.extra_values
            )
        }
    }
}

impl JSON for Data {
    fn to_json(&self) -> String {
        match self {
            Data::Boolean(true) => format!("true"),
            Data::Boolean(false) => format!("false"),
            Data::Nil => format!("null"),
            Data::Object(o) => o.to_json(),
            Data::Bytes(bytes) => match String::from_utf8(bytes.to_vec()) {
                Ok(string) => format!("{{ \"bytes\": {:?}, \"string\": \"{}\" }}", bytes, string),
                _ => format!("{{ \"bytes\": {:?} }}", bytes),
            },
            Data::Int8(val) => format!("{}", val),
            Data::Int16(val) => format!("{}", val),
            Data::Int32(val) => format!("{}", val),
            Data::Int64(val) => format!("{}", val),
            Data::Float(val) => format!("{}", val),
            Data::Double(val) => format!("{}", val),
        }
    }
}
