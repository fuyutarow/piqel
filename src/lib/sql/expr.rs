use ordered_float::OrderedFloat;
use rayon::vec;

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
    Mod(Box<Expr>, Box<Expr>),
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
            Self::Mod(left, right) => Self::Mod(
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

    pub fn eval_to_vector(self, book: &FieldBook) -> PqlVector {
        match self.to_owned() {
            Expr::Path(path) => {
                let v = book
                    .source_fields
                    .get(&path.last().unwrap())
                    .unwrap()
                    .to_owned();
                PqlVector(v)
            }
            Self::Num(num) => PqlVector(vec![PqlValue::Float(OrderedFloat(num)); book.column_size]),
            Self::Add(box expr1, box expr2) => {
                expr1.eval_to_vector(&book) + expr2.eval_to_vector(&book)
            }
            Self::Mul(box expr1, box expr2) => {
                expr1.eval_to_vector(&book) * expr2.eval_to_vector(&book)
            }
            _ => {
                dbg!(&self);

                todo!()
            }
        }
    }

    pub fn eval(self) -> PqlValue {
        match self.to_owned() {
            Self::Num(num) => PqlValue::Float(OrderedFloat(num.to_owned())),
            Self::Add(box expr1, box expr2) => (expr1).eval() + (expr2).eval(),
            Self::Sub(box expr1, box expr2) => (expr1).eval() - (expr2).eval(),
            Self::Mul(box expr1, box expr2) => (expr1).eval() * (expr2).eval(),
            Self::Div(box expr1, box expr2) => (expr1).eval() / (expr2).eval(),
            Self::Mod(box expr1, box expr2) => (expr1).eval() % (expr2).eval(),
            Self::Exp(box expr1, box expr2) => (expr1).eval().powf((expr2).eval()),
            _ => {
                dbg!(&self);

                todo!()
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    Count(Expr),
    Upper(Expr),
}
