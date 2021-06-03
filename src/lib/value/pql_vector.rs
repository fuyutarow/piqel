use std::collections::{BTreeMap, BTreeSet};
use std::ops::{Add, Div, Mul, Neg, Sub};

use indexmap::IndexMap;
use ordered_float::OrderedFloat;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::value::PqlValue;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct PqlVector(pub Vec<PqlValue>);

impl Neg for PqlVector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        let v = self.0.into_iter().map(|value| -value).collect::<Vec<_>>();
        Self(v)
    }
}

impl Add for PqlVector {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        let v = self
            .0
            .into_iter()
            .zip(other.0.into_iter())
            .map(|(a, b)| a + b)
            .collect::<Vec<PqlValue>>();
        Self(v)
    }
}

impl Mul for PqlVector {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        let v = self
            .0
            .into_iter()
            .zip(other.0.into_iter())
            .map(|(a, b)| a * b)
            .collect::<Vec<PqlValue>>();
        Self(v)
    }
}
