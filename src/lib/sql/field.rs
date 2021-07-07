use std::str::FromStr;

use crate::parser;
use crate::sql::Env;
use crate::sql::Expr;
use crate::value::PqlValue;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Field {
    pub expr: Expr,
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
            expr: env.expand_fullpath(&self.expr),
            alias: self.alias.to_owned(),
        }
    }

    pub fn evaluate(self, env: &Env) -> PqlValue {
        let value = self.expr.eval(&env);
        value
    }

    pub fn rename(self) -> (String, Expr) {
        if let Some(alias) = self.alias {
            (alias, self.expr)
        } else {
            let alias = match &self.expr {
                Expr::Selector(selector) => selector.to_vec().last().unwrap().to_string(),
                _ => todo!(),
            };
            (alias, self.expr)
        }
    }
}
