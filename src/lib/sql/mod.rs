use crate::value::PqlValue;

mod bindings;
mod eval;
mod expr;
mod field;
pub mod parser;
mod utils;
mod where_cond;

pub use bindings::Bindings;
pub use eval::{run, to_list};
pub use expr::Expr;
pub use field::{DPath, Field};
pub use where_cond::DWhereCond;

#[derive(Debug, Clone, PartialEq)]
pub struct Proj {
    pub expr: Expr,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sql {
    pub select_clause: Vec<Proj>,
    pub from_clause: Vec<Field>,
    pub left_join_clause: Vec<Field>,
    pub where_clause: Option<DWhereCond>,
}
