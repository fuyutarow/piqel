use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::sql::DPath;
use crate::sql::Field;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BPqlValue {
    Null,
    Str(String),
    Boolean(bool),
    Float(OrderedFloat<f64>),
    Int(i64),
    Array(BTreeSet<Self>),
    Object(BTreeMap<String, Self>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PqlValue {
    Null,
    Str(String),
    Boolean(bool),
    Int(i64),
    Float(OrderedFloat<f64>),
    Array(Vec<Self>),
    Object(IndexMap<String, Self>),
}

impl From<&str> for PqlValue {
    fn from(s: &str) -> Self {
        Self::Str(s.to_owned())
    }
}

impl From<bool> for PqlValue {
    fn from(b: bool) -> Self {
        Self::Boolean(b)
    }
}

impl From<i64> for PqlValue {
    fn from(i: i64) -> Self {
        Self::Int(i)
    }
}

impl From<f64> for PqlValue {
    fn from(f: f64) -> Self {
        Self::Float(OrderedFloat(f))
    }
}

impl Default for PqlValue {
    fn default() -> Self {
        Self::Null
    }
}

impl PqlValue {
    pub fn get(self, key: &str) -> Option<Self> {
        match self {
            Self::Object(map) => map.get(key).map(|v| v.to_owned()),
            _ => None,
        }
    }

    pub fn get_path(self, path: &[&str]) -> Option<Self> {
        if let Some((key, path)) = path.split_first() {
            if let Some(obj) = self.get(key) {
                if path.is_empty() {
                    Some(obj)
                } else {
                    obj.get_path(path)
                }
            } else {
                None
            }
        } else {
            unreachable!();
        }
    }

    pub fn select_by_path(&self, path: &DPath) -> Option<Self> {
        match self {
            Self::Object(map) => {
                if let Some((key, tail_path)) = path.to_vec().split_first() {
                    if let Some(obj) = self.clone().get(key) {
                        obj.select_by_path(&DPath::from(tail_path))
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

                Some(Self::Array(new_array))
            }
            _ => Some(self.clone()),
        }
    }

    // pub fn select_by_paths(&self, path: &[DPath]) -> Vec<Self> {

    //     match self {
    //         Self::Object(map) => {
    //             if let Some((key, tail_path)) = path.to_vec().split_first() {
    //                 if let Some(obj) = self.clone().get(key) {
    //                     obj.select_by_path(&DPath::from(tail_path))
    //                 } else {
    //                     None
    //                 }
    //             } else {
    //                 Some(self.to_owned())
    //             }
    //         }
    //         Self::Array(array) => {
    //             let new_array = array
    //                 .into_iter()
    //                 .filter_map(|value| value.select_by_path(&path))
    //                 .collect::<Vec<_>>();

    //             Some(Self::Array(new_array))
    //         }
    //         _ => Some(self.clone()),
    //     }
    // }

    pub fn select_by_fields(&self, field_list: &[Field]) -> Option<Self> {
        let mut new_map = IndexMap::<String, Self>::new();

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

        Some(Self::Object(new_map))
    }

    pub fn select_map_by_fields(&self, field_list: &[Field]) -> Option<Self> {
        match self {
            Self::Array(array) => {
                let new_array = array
                    .into_iter()
                    .filter_map(|value| value.select_by_fields(field_list))
                    .collect::<Vec<_>>();

                Some(Self::Array(new_array))
            }
            _ => None,
        }
    }
}

impl Neg for PqlValue {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            Self::Int(a) => Self::Int(-a),
            Self::Float(a) => Self::Float(-a),
            _ => todo!(),
        }
    }
}

impl Add for PqlValue {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a + b),
            (Self::Int(a), Self::Float(b)) => Self::Float(OrderedFloat(a as f64) + b),
            (Self::Float(a), Self::Int(b)) => Self::Float(a + OrderedFloat(b as f64)),
            (Self::Float(a), Self::Float(b)) => Self::Float(a + b),
            _ => todo!(),
        }
    }
}

impl Sub for PqlValue {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a - b),
            (Self::Int(a), Self::Float(b)) => Self::Float(OrderedFloat(a as f64) - b),
            (Self::Float(a), Self::Int(b)) => Self::Float(a - OrderedFloat(b as f64)),
            (Self::Float(a), Self::Float(b)) => Self::Float(a - b),
            _ => todo!(),
        }
    }
}

impl Mul for PqlValue {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        match (self.to_owned(), other.to_owned()) {
            (Self::Int(a), Self::Int(b)) => Self::Int(a * b),
            (Self::Int(a), Self::Float(b)) => Self::Float(OrderedFloat(a as f64) * b),
            (Self::Float(a), Self::Int(b)) => Self::Float(a * OrderedFloat(b as f64)),
            (Self::Float(a), Self::Float(b)) => Self::Float(a * b),
            _ => {
                dbg!(&self, &other);
                todo!()
            }
        }
    }
}

impl Div for PqlValue {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Self::Int(a), Self::Int(b)) => Self::Float(OrderedFloat(a as f64 / b as f64)),
            (Self::Int(a), Self::Float(b)) => Self::Float(OrderedFloat(a as f64) / b),
            (Self::Float(a), Self::Int(b)) => Self::Float(a / OrderedFloat(b as f64)),
            (Self::Float(a), Self::Float(b)) => Self::Float(a / b),
            _ => todo!(),
        }
    }
}

impl Rem for PqlValue {
    type Output = Self;
    fn rem(self, other: Self) -> Self::Output {
        let (a, b) = match (self, other) {
            (Self::Int(a), Self::Int(b)) => (a as f64, b as f64),
            (Self::Int(a), Self::Float(OrderedFloat(b))) => (a as f64, b),
            (Self::Float(OrderedFloat(a)), Self::Int(b)) => (a, b as f64),
            (Self::Float(OrderedFloat(a)), Self::Float(OrderedFloat(b))) => (a, b),
            _ => todo!(),
        };
        Self::from(a % b)
    }
}

impl PqlValue {
    pub fn powf(self, other: Self) -> Self {
        let (a, b) = match (self, other) {
            (Self::Int(a), Self::Int(b)) => (a as f64, b as f64),
            (Self::Int(a), Self::Float(OrderedFloat(b))) => (a as f64, b),
            (Self::Float(OrderedFloat(a)), Self::Int(b)) => (a, b as f64),
            (Self::Float(OrderedFloat(a)), Self::Float(OrderedFloat(b))) => (a, b),
            _ => todo!(),
        };
        Self::from(a.powf(b))
    }
}

impl TryFrom<PqlValue> for i64 {
    type Error = anyhow::Error;
    fn try_from(value: PqlValue) -> anyhow::Result<Self> {
        match value {
            PqlValue::Int(int) => Ok(int),
            PqlValue::Float(OrderedFloat(f)) => Ok(f as i64),
            _ => anyhow::bail!("not numeric"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::PqlValue;
    use ordered_float::OrderedFloat;

    #[test]
    fn add_sub_mul_div() {
        assert_eq!(
            PqlValue::Float(OrderedFloat(1.)) + PqlValue::Float(OrderedFloat(2.)),
            PqlValue::Float(OrderedFloat(3.))
        );
        assert_eq!(
            PqlValue::Float(OrderedFloat(1.)) / PqlValue::Float(OrderedFloat(0.)),
            PqlValue::Float(OrderedFloat(f64::INFINITY))
        );
    }
}
