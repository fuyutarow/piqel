use std::collections::VecDeque;

use crate::sql::Bindings;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorNode {
    String(String),
    Number(i64),
}

impl From<SelectorNode> for String {
    fn from(node: SelectorNode) -> Self {
        match node {
            SelectorNode::String(s) => s,
            SelectorNode::Number(i) => format!("{}", i),
        }
    }
}

impl SelectorNode {
    pub fn to_string(&self) -> String {
        String::from(self.to_owned())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Selector {
    pub data: VecDeque<SelectorNode>,
}

impl From<&[&str]> for Selector {
    fn from(ss: &[&str]) -> Self {
        let data = ss
            .iter()
            .map(|s| SelectorNode::String(s.to_string()))
            .collect::<VecDeque<_>>();
        Self { data }
    }
}

impl From<&[String]> for Selector {
    fn from(ss: &[String]) -> Self {
        let data = ss
            .iter()
            .map(|s| SelectorNode::String(s.to_string()))
            .collect::<VecDeque<_>>();
        Self { data }
    }
}

impl From<&str> for Selector {
    fn from(s: &str) -> Self {
        let data = s
            .to_string()
            .split(".")
            .map(|s| SelectorNode::String(s.to_string()))
            .collect::<VecDeque<_>>();
        Self { data }
    }
}

impl Selector {
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
        self.to_vec().join(".")
    }

    pub fn to_vec(&self) -> Vec<String> {
        self.data
            .clone()
            .into_iter()
            .map(String::from)
            .collect::<Vec<String>>()
    }

    pub fn expand_fullpath(&self, bidings: &Bindings) -> Self {
        bidings.get_full_path(&self)
    }
}
