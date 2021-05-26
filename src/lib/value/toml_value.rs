use indexmap::IndexMap as Map;
use ordered_float::OrderedFloat;
use serde_derive::{Deserialize, Serialize};

use crate::value::PqlValue;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TomlValue {
    #[serde(skip_serializing)]
    Null,
    Str(String),
    Boolean(bool),
    Float(OrderedFloat<f64>),
    Int(i64),
    Array(Vec<Self>),
    Object(Map<String, Self>),
}

impl From<PqlValue> for TomlValue {
    fn from(pqlv: PqlValue) -> Self {
        match pqlv {
            PqlValue::Null => Self::Null,
            PqlValue::Str(string) => Self::Str(string),
            PqlValue::Boolean(boolean) => Self::Boolean(boolean),
            PqlValue::Float(float) => Self::Float(float),
            PqlValue::Int(int) => Self::Int(int),
            PqlValue::Array(array) => Self::Array(
                array
                    .into_iter()
                    .filter_map(|v| match v {
                        PqlValue::Null => None,
                        _ => Some(Self::from(v)),
                    })
                    .collect::<Vec<_>>(),
            ),
            PqlValue::Object(map) => Self::Object(
                map.into_iter()
                    .filter_map(|(k, v)| match v {
                        PqlValue::Null => None,
                        _ => Some((k, Self::from(v))),
                    })
                    .collect::<Map<_, _>>(),
            ),
        }
    }
}
