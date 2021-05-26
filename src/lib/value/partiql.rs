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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BJsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(OrderedFloat<f64>),
    Array(BTreeSet<BJsonValue>),
    Object(BTreeMap<String, BJsonValue>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonValue {
    Null,
    Str(String),
    Boolean(bool),
    Num(OrderedFloat<f64>),
    Array(Vec<JsonValue>),
    Object(IndexMap<String, JsonValue>),
}

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

    pub fn get_path(self, path: &[&str]) -> Option<JsonValue> {
        if let Some((key, path)) = path.split_first() {
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

    pub fn select_by_path(&self, path: &Dpath) -> Option<JsonValue> {
        match self {
            Self::Object(map) => {
                if let Some((key, tail_path)) = path.to_vec().split_first() {
                    if let Some(obj) = self.clone().get(key) {
                        obj.select_by_path(&Dpath::from(tail_path))
                    } else {
                        None
                    }
                } else {
                    Some(self.to_owned())
                }
            }
            Self::Array(array) => {
                let new_array = array
                    .into_iter()
                    .filter_map(|value| value.select_by_path(&path))
                    .collect::<Vec<_>>();

                Some(JsonValue::Array(new_array))
            }
            _ => Some(self.clone()),
        }
    }

    pub fn select_by_fields(&self, field_list: &[DField]) -> Option<JsonValue> {
        let mut new_map = IndexMap::<String, JsonValue>::new();

        for field in field_list {
            if let Some(value) = self.select_by_path(&field.path) {
                let key = field.alias.clone().unwrap_or({
                    let last = field.path.to_vec().last().unwrap().to_string();
                    last
                });
                new_map.insert(key, value);
            } else {
            }
        }

        Some(JsonValue::Object(new_map))
    }

    pub fn select_map_by_fields(&self, field_list: &[DField]) -> Option<JsonValue> {
        match self {
            JsonValue::Array(array) => {
                let new_array = array
                    .into_iter()
                    .filter_map(|value| value.select_by_fields(field_list))
                    .collect::<Vec<_>>();

                Some(JsonValue::Array(new_array))
            }
            _ => None,
        }
    }
}
