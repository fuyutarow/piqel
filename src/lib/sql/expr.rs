use std::collections::HashSet;

use collect_mac::collect;
use ordered_float::OrderedFloat;


use crate::sql::Bindings;
use crate::sql::DPath;
use crate::sql::FieldBook;
use crate::sql::Sql;
use crate::value::PqlValue;
use crate::value::PqlVector;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Path(DPath),
    Num(f64),
    Func(Box<Func>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>, Box<Expr>),
    Sql(Sql),
}

impl Default for Expr {
    fn default() -> Self {
        Self::Num(0.)
    }
}

impl Expr {
    pub fn as_path(&self) -> Option<DPath> {
        match self {
            Expr::Path(path) => Some(path.to_owned()),
            _ => None,
        }
    }

    pub fn expand_fullpath(&self, bindings: &Bindings) -> Self {
        match self {
            Self::Path(path) => Self::Path(path.expand_fullpath(&bindings)),
            Self::Num(_) => self.to_owned(),
            Self::Add(left, right) => Self::Add(
                Box::new((*left).expand_fullpath(&bindings)),
                Box::new((*right).expand_fullpath(&bindings)),
            ),
            Self::Sub(left, right) => Self::Sub(
                Box::new((*left).expand_fullpath(&bindings)),
                Box::new((*right).expand_fullpath(&bindings)),
            ),
            Self::Mul(left, right) => Self::Mul(
                Box::new((*left).expand_fullpath(&bindings)),
                Box::new((*right).expand_fullpath(&bindings)),
            ),
            Self::Div(left, right) => Self::Div(
                Box::new((*left).expand_fullpath(&bindings)),
                Box::new((*right).expand_fullpath(&bindings)),
            ),
            Self::Rem(left, right) => Self::Rem(
                Box::new((*left).expand_fullpath(&bindings)),
                Box::new((*right).expand_fullpath(&bindings)),
            ),
            Self::Exp(left, right) => Self::Exp(
                Box::new((*left).expand_fullpath(&bindings)),
                Box::new((*right).expand_fullpath(&bindings)),
            ),
            _ => todo!(),
        }
    }

    pub fn eval_to_vector(self, book: &FieldBook, bindings: &Bindings) -> PqlVector {
        match self.to_owned() {
            Expr::Path(path) => {
                let path = path.expand_fullpath(&bindings);
                let v = book
                    .source_fields
                    .get(path.to_string().as_str())
                    .unwrap()
                    .to_owned();
                PqlVector(v)
            }
            Self::Num(num) => PqlVector(vec![PqlValue::Float(OrderedFloat(num)); book.column_size]),
            Self::Add(box expr1, box expr2) => {
                expr1.eval_to_vector(&book, &bindings) + expr2.eval_to_vector(&book, &bindings)
            }
            Self::Sub(box expr1, box expr2) => {
                expr1.eval_to_vector(&book, &bindings) - expr2.eval_to_vector(&book, &bindings)
            }
            Self::Mul(box expr1, box expr2) => {
                expr1.eval_to_vector(&book, &bindings) * expr2.eval_to_vector(&book, &bindings)
            }
            Self::Div(box expr1, box expr2) => {
                expr1.eval_to_vector(&book, &bindings) / expr2.eval_to_vector(&book, &bindings)
            }
            Self::Rem(box expr1, box expr2) => {
                expr1.eval_to_vector(&book, &bindings) % expr2.eval_to_vector(&book, &bindings)
            }
            _ => todo!(),
        }
    }

    pub fn eval(self) -> PqlValue {
        match self.to_owned() {
            Self::Num(num) => PqlValue::Float(OrderedFloat(num.to_owned())),
            Self::Add(box expr1, box expr2) => (expr1).eval() + (expr2).eval(),
            Self::Sub(box expr1, box expr2) => (expr1).eval() - (expr2).eval(),
            Self::Mul(box expr1, box expr2) => (expr1).eval() * (expr2).eval(),
            Self::Div(box expr1, box expr2) => (expr1).eval() / (expr2).eval(),
            Self::Rem(box expr1, box expr2) => (expr1).eval() % (expr2).eval(),
            Self::Exp(box expr1, box expr2) => (expr1).eval().powf((expr2).eval()),
            _ => {
                dbg!(&self);

                todo!()
            }
        }
    }

    pub fn source_field_name_set(&self, bindings: &Bindings) -> HashSet<String> {
        match self.to_owned() {
            Expr::Num(_) => HashSet::default(),
            Expr::Path(path) => {
                collect! {
                    as HashSet<String>:
                    path.expand_fullpath(&bindings).to_string()
                }
            }
            Expr::Add(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&bindings);
                let b = expr2.source_field_name_set(&bindings);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Sub(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&bindings);
                let b = expr2.source_field_name_set(&bindings);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Mul(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&bindings);
                let b = expr2.source_field_name_set(&bindings);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Div(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&bindings);
                let b = expr2.source_field_name_set(&bindings);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Rem(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&bindings);
                let b = expr2.source_field_name_set(&bindings);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Exp(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&bindings);
                let b = expr2.source_field_name_set(&bindings);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            _ => {
                dbg!(&self);

                todo!();
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    Count(Expr),
    Upper(Expr),
}
