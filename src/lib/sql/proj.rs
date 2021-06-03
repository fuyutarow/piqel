use ordered_float::OrderedFloat;

use crate::sql::Bindings;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::FieldBook;
use crate::value::PqlValue;
use crate::value::PqlVector;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Proj {
    pub expr: Expr,
    pub alias: Option<String>,
}

impl Proj {
    pub fn eval(self, book: &FieldBook) -> PqlVector {
        match self.expr {
            Expr::Path(path) => {
                let v = book
                    .source_fields
                    .get(&path.last().unwrap())
                    .unwrap()
                    .to_owned();
                PqlVector(v)
            }
            Expr::Num(float) => {
                PqlVector(vec![PqlValue::Float(OrderedFloat(float)); book.column_size])
            }
            Expr::Mul(box left, box right) => {
                left.eval_to_vector(&book) * right.eval_to_vector(&book)
            }
            // Expr::Add(_)  =>
            // | Expr::Sub(_)
            _ => {
                dbg!(self.expr);
                todo!();
            }
        }
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
