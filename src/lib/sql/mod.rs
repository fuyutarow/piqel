mod env;
mod expr;
mod field;
mod filter;
mod selector;
mod utils;
mod where_cond;

pub use env::Env;
pub use expr::{Expr, Func};
pub use field::Field;
pub use field::SourceValue;
pub use filter::restrict;
pub use selector::Selector;
pub use selector::SelectorNode;
pub use where_cond::re_from_str;
pub use where_cond::WhereCond;

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
