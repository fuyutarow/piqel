use std::str::FromStr;

use crate::parser;

use crate::sql::Env;
use crate::sql::Expr;
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
    pub fn expand_fullpath(&self, env: &Env) -> Self {
        Self {
            value: env.expand_fullpath(&self.value),
            alias: self.alias.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SourceValue {
    Selector(Selector),
    Value(PqlValue),
    Expr(Expr),
}

impl Default for SourceValue {
    fn default() -> Self {
        Self::Value(PqlValue::default())
    }
}

impl SourceValue {
    pub fn to_string(self) -> String {
        match self {
            Self::Selector(selector) => selector.to_string(),
            Self::Value(value) => value.to_json().expect("to json"),
            _ => todo!(),
        }
    }
}
