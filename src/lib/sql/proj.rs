use std::collections::HashSet;

use ordered_float::OrderedFloat;

use crate::sql::Bindings;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::FieldBook;
use crate::value::json_value::to_pqlvalue;
use crate::value::PqlValue;
use crate::value::PqlVector;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Proj {
    pub expr: Expr,
    pub alias: Option<String>,
}

impl Proj {
    pub fn eval(self, book: &FieldBook) -> PqlVector {
        self.expr.eval_to_vector(&book)
    }

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

    pub fn source_field_name_set(&self) -> HashSet<String> {
        self.expr.source_field_name_set()
    }

    pub fn target_field_name(&self) -> String {
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