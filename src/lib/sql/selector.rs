use std::collections::VecDeque;
use std::str::FromStr;

use itertools::any;

use crate::parser;
use crate::sql::Bindings;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorNode {
    String(String),
    Number(i64),
}

impl Default for SelectorNode {
    fn default() -> Self {
        Self::String(String::default())
    }
}

impl From<&str> for SelectorNode {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<i64> for SelectorNode {
    fn from(i: i64) -> Self {
        Self::Number(i)
    }
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

impl FromStr for Selector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match parser::expressions::parse_selector(s) {
            Ok((_, r)) => Ok(r),
            Err(_err) => anyhow::bail!("failed"),
        }
    }
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

impl From<&[SelectorNode]> for Selector {
    fn from(nodes: &[SelectorNode]) -> Self {
        Self {
            data: nodes
                .into_iter()
                .map(|n| n.to_owned())
                .collect::<VecDeque<_>>(),
        }
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

    pub fn split_first(&self) -> Option<(SelectorNode, Self)> {
        let mut data = self.data.to_owned();

        if let Some(first) = data.pop_front() {
            Some((first, Self { data }))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        self.data
            .clone()
            .into_iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn to_vec(&self) -> Vec<SelectorNode> {
        self.data.clone().into_iter().collect::<Vec<SelectorNode>>()
    }

    pub fn expand_fullpath(&self, bidings: &Bindings) -> Self {
        bidings.get_full_path(&self)
    }
}
