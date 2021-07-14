use crate::sql::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    Count(Expr),
    Upper(Expr),
}
