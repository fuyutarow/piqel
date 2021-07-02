use std::collections::{BTreeMap, BTreeSet};
use std::convert::TryFrom;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::str::FromStr;

use chrono::prelude::*;
use chrono::serde::ts_seconds;
use indexmap::IndexMap;
use itertools::any;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::sql::Selector;
use crate::sql::SelectorNode;

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BPqlValue {
    Null,
    Str(String),
    Boolean(bool),
    Float(OrderedFloat<f64>),
    Int(i64),
    #[serde(with = "ts_seconds")]
    DateTime(DateTime<Utc>),
    Array(BTreeSet<Self>),
    Object(BTreeMap<String, Self>),
}

impl From<PqlValue> for BPqlValue {
    fn from(pqlv: PqlValue) -> Self {
        match pqlv {
            PqlValue::Null => Self::Null,
            PqlValue::Str(s) => Self::Str(s),
            PqlValue::Boolean(b) => Self::Boolean(b),
            PqlValue::Int(i) => Self::Int(i),
            PqlValue::Float(f) => Self::Float(f),
            PqlValue::DateTime(t) => Self::DateTime(t),
            PqlValue::Array(_) => todo!(),
            PqlValue::Object(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PqlValue {
    Null,
    Str(String),
    Boolean(bool),
    Int(i64),
    Float(OrderedFloat<f64>),
    #[serde(with = "ts_seconds")]
    DateTime(DateTime<Utc>),
    Array(Vec<Self>),
    Object(IndexMap<String, Self>),
}

impl Default for PqlValue {
    fn default() -> Self {
        Self::Null
    }
}

impl FromStr for PqlValue {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        crate::pqlir_parser::from_str(s)
    }
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

impl From<Vec<PqlValue>> for PqlValue {
    fn from(v: Vec<PqlValue>) -> Self {
        Self::Array(v)
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

    pub fn select_by_key(&self, key: &SelectorNode) -> Option<Self> {
        match (self, key.to_owned()) {
            (Self::Object(map), SelectorNode::String(key_s)) => {
                map.get(&key_s).map(|v| v.to_owned())
            }
            _ => None,
        }
    }

    pub fn select_by_selector(&self, selector: &Selector) -> Option<Self> {
        match self {
            Self::Object(_map) => {
                if let Some((key, tail)) = selector.split_first_old() {
                    if let Some(obj) = self.select_by_key(&key) {
                        obj.select_by_selector(&tail)
                    } else {
                        None
                    }
                } else {
                    Some(self.to_owned())
                }
            }
            Self::Array(array) => {
                if let Some((key, _tail)) = selector.split_first_old() {
                    match key {
                        SelectorNode::Number(key_i) => {
                            if key_i < 0 {
                                todo!()
                            } else {
                                let key_u = key_i as usize;
                                array.get(key_u).map(|v| v.to_owned())
                            }
                        }
                        _ => {
                            let new_array = array
                                .into_iter()
                                .filter_map(|value| value.select_by_selector(&selector))
                                .collect::<Vec<_>>();
                            Some(Self::Array(new_array))
                        }
                    }
                } else {
                    let new_array = array
                        .into_iter()
                        .filter_map(|value| value.select_by_selector(&selector))
                        .collect::<Vec<_>>();
                    Some(Self::Array(new_array))
                }
            }
            _ => Some(self.clone()),
        }
    }

    pub fn print(&self) -> anyhow::Result<()> {
        println!("{}", self.to_json()?);
        Ok(())
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        self.to_jsonp()
    }

    pub fn to_jsonp(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }

    pub fn to_jsonc(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
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
    use std::collections::VecDeque;
    use std::str::FromStr;

    use ordered_float::OrderedFloat;

    use super::PqlValue;
    use crate::pqlir_parser;
    use crate::sql::Selector;
    use crate::sql::SelectorNode;

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

    #[test]
    fn select_at_arr_1() -> anyhow::Result<()> {
        let value = PqlValue::from_str(r#"{ "arr" : [1,2,4] }"#)?;

        let selected_value = value.select_by_selector(&Selector {
            data: vec![
                SelectorNode::String(String::from("arr")),
                SelectorNode::Number(1),
            ]
            .into_iter()
            .collect::<VecDeque<SelectorNode>>(),
        });

        assert_eq!(selected_value, Some(pqlir_parser::from_str("2")?));
        Ok(())
    }
}
