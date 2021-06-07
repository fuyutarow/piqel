use crate::value::PqlValue;

mod bindings;
mod eval;
mod expr;
mod field;
mod filter;
mod proj;
mod utils;
mod where_cond;

pub use bindings::Bindings;
pub use eval::evaluate;
pub use eval::to_list;
pub use eval::FieldBook;
pub use expr::{Expr, Func};
pub use field::{DPath, Field};
pub use filter::restrict;
pub use proj::Proj;
pub use where_cond::re_from_str;
pub use where_cond::WhereCond;

#[derive(Debug, Clone, PartialEq)]
pub struct Sql {
    pub select_clause: Vec<Proj>,
    pub from_clause: Vec<Field>,
    pub left_join_clause: Vec<Field>,
    pub where_clause: Option<Box<WhereCond>>,
    pub orderby: Option<clause::OrderBy>,
    pub limit: Option<clause::Limit>,
}

impl Sql {
    pub fn get_colnames(&self) -> Vec<String> {
        self.select_clause
            .iter()
            .map(|proj| proj.target_field_name())
            .collect()
    }
}

pub mod clause {
    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct OrderBy {
        pub label: String,
        pub is_asc: bool,
    }

    #[derive(Debug, Default, Clone, PartialEq)]
    pub struct Limit {
        pub limit: u64,
        pub offset: u64,
    }
}
