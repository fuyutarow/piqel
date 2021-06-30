use std::str::FromStr;

use crate::parser;
use crate::sql::Bindings;
use crate::sql::Selector;
use crate::value::PqlValue;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Field {
    pub value: SourceValue,
    pub alias: Option<String>,
}

impl FromStr for Field {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match parser::expressions::parse_field(s) {
            Ok((_, field)) => Ok(field),
            Err(nom::Err::Error(err)) => {
                eprint!("{}", err);
                anyhow::bail!("failed")
            }
            _ => todo!(),
        }
    }
}

impl Field {
    pub fn expand_fullpath(&self, bindings: &Bindings) -> Self {
        let value = match &self.value {
            SourceValue::Selector(selector) => {
                SourceValue::Selector(selector.expand_fullpath(&bindings))
            }
            SourceValue::Value(value) => self.value.to_owned(),
        };
        Self {
            value,
            alias: self.alias.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceValue {
    Selector(Selector),
    Value(PqlValue),
}

impl Default for SourceValue {
    fn default() -> Self {
        Self::Value(PqlValue::default())
    }
}
