use crate::value::PqlValue;

mod bindings;
mod eval;
mod expr;
mod field;
mod filter;
mod utils;
mod where_cond;

pub use bindings::Bindings;
pub use eval::FieldBook;
pub use eval::{evaluate, to_list};
pub use expr::{Expr, Func};
pub use field::{DPath, Field};
pub use filter::restrict;
pub use where_cond::re_from_str;
pub use where_cond::WhereCond;

#[derive(Debug, Clone, PartialEq)]
pub struct Proj {
    pub expr: Expr,
    pub alias: Option<String>,
}

impl Proj {
    pub fn to_field(&self, bindings: &Bindings) -> Field {
        let expr = self.expr.expand_fullpath(&bindings);
        match expr {
            Expr::Path(path) => Field {
                path,
                alias: self.alias.to_owned(),
            },
            _ => {
                dbg!(expr);
                todo!();
            }
        }
    }

    pub fn get_colname(&self) -> String {
        if let Some(alias) = self.alias.to_owned() {
            alias
        } else {
            match self.expr.to_owned() {
                Expr::Path(path) => path.to_vec().last().unwrap().to_string(),
                _ => {
                    todo!();
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sql {
    pub select_clause: Vec<Proj>,
    pub from_clause: Vec<Field>,
    pub left_join_clause: Vec<Field>,
    pub where_clause: Option<Box<WhereCond>>,
}

impl Sql {
    pub fn get_colnames(&self) -> Vec<String> {
        self.select_clause
            .iter()
            .map(|proj| proj.get_colname())
            .collect()
    }
}
