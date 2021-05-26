use std::collections::HashMap;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use serde_derive::{Deserialize, Serialize};
use toml::value::Value as TomlValue;

use crate::sql::DField;
use crate::sql::Dpath;
use crate::sql::Field;

pub mod json;
mod partiql;
pub use json::{BJsonValue, JsonValue};
pub use partiql::{BPqlValue, PqlValue};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonValueForToml {
    #[serde(skip_serializing)]
    Null,
    Str(String),
    Boolean(bool),
    Num(OrderedFloat<f64>),
    Array(Vec<Self>),
    Object(IndexMap<String, Self>),
}

impl From<JsonValue> for JsonValueForToml {
    fn from(json: JsonValue) -> Self {
        match json {
            JsonValue::Null => Self::Null,
            JsonValue::Str(string) => Self::Str(string),
            JsonValue::Boolean(boolean) => Self::Boolean(boolean),
            JsonValue::Num(number) => Self::Num(number),
            JsonValue::Array(array) => Self::Array(
                array
                    .into_iter()
                    .filter_map(|v| match v {
                        JsonValue::Null => None,
                        _ => Some(Self::from(v)),
                    })
                    .collect::<Vec<_>>(),
            ),
            JsonValue::Object(map) => Self::Object(
                map.into_iter()
                    .filter_map(|(k, v)| match v {
                        JsonValue::Null => None,
                        _ => Some((k, Self::from(v))),
                    })
                    .collect::<IndexMap<_, _>>(),
            ),
        }
    }
}
