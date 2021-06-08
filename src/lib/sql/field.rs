use collect_mac::collect;

use crate::sql::Bindings;
use crate::sql::Expr;
use std::collections::VecDeque;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Field {
    pub path: DPath,
    pub alias: Option<String>,
}

impl Field {
    pub fn expand_fullpath(&self, bindings: &Bindings) -> Self {
        Self {
            path: self.path.expand_fullpath(&bindings),
            alias: self.alias.to_owned(),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct DPath {
    pub data: VecDeque<String>,
}

impl From<&[&str]> for DPath {
    fn from(ss: &[&str]) -> Self {
        let data = ss.iter().map(|s| s.to_string()).collect::<VecDeque<_>>();
        Self { data }
    }
}

impl From<&str> for DPath {
    fn from(s: &str) -> Self {
        let data = s
            .to_string()
            .split(".")
            .map(|s| s.to_string())
            .collect::<VecDeque<_>>();
        Self { data }
    }
}

impl DPath {
    pub fn last(&self) -> Option<String> {
        if let Some(last) = self.to_vec().last() {
            Some(last.to_string())
        } else {
            None
        }
    }

    pub fn split_first(&self) -> Option<(Self, Self)> {
        let mut data = self.data.clone();

        if let Some(first) = data.pop_front() {
            let mut vec = VecDeque::new();
            vec.push_back(first);
            Some((Self { data: vec }, Self { data }))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        self.data
            .clone()
            .into_iter()
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.data.iter().map(|s| s.as_str()).collect::<Vec<_>>()
    }

    pub fn expand_fullpath(&self, bidings: &Bindings) -> Self {
        bidings.get_full_path(&self)
    }
}
