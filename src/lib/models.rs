use std::collections::HashMap;
use std::fmt;

use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(f64),
    Array(Vec<JsonValue>),
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn get(self, key: &str) -> Option<JsonValue> {
        match self {
            JsonValue::Object(map) => {
                if let Some(value) = map.get(key) {
                    Some(value.to_owned())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn get_path(self, path: &[String]) -> Option<JsonValue> {
        if let Some((key, path)) = path.split_first() {
            dbg!(&key);
            if let Some(obj) = self.get(key) {
                if path.len() > 0 {
                    obj.get_path(path)
                } else {
                    Some(obj)
                }
            } else {
                None
            }
        } else {
            unreachable!();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    data: u64,
}

impl From<u64> for Atom {
    fn from(v: u64) -> Self {
        Self { data: v }
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{{");
        writeln!(f, "  '_1': {},", self.data);
        writeln!(f, "}}");
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    data: Vec<Atom>,
}

impl From<&[u64]> for Array {
    fn from(v: &[u64]) -> Self {
        Self {
            data: v.to_vec().into_iter().map(Atom::from).collect::<Vec<_>>(),
        }
    }
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "<<");
        for atom in self.data.iter() {
            writeln!(f, "  {},", atom);
        }
        writeln!(f, ">>");
        Ok(())
    }
}
