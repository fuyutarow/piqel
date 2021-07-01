use std::collections::HashSet;

use collect_mac::collect;
use ordered_float::OrderedFloat;

use crate::planner::Sql;
use crate::sql::Env;
use crate::sql::Selector;
use crate::value::PqlValue;
use crate::value::PqlVector;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Star,
    Selector(Selector),
    Value(PqlValue),
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
        Self::Value(PqlValue::default())
    }
}

impl From<Expr> for String {
    fn from(expr: Expr) -> Self {
        match expr {
            Expr::Selector(selector) => selector.to_string(),
            Expr::Value(value) => value.to_json().expect("to json"),
            _ => todo!(),
        }
    }
}

impl Expr {
    pub fn to_string(self) -> String {
        String::from(self)
    }

    pub fn as_path(&self) -> Option<Selector> {
        match self {
            Expr::Selector(path) => Some(path.to_owned()),
            _ => None,
        }
    }

    pub fn expand_fullpath(&self, env: &Env) -> Self {
        match self {
            Self::Selector(path) => Self::Selector(path.expand_fullpath2(&env)),
            Self::Num(_) => self.to_owned(),
            Self::Add(left, right) => Self::Add(
                Box::new((*left).expand_fullpath(&env)),
                Box::new((*right).expand_fullpath(&env)),
            ),
            Self::Sub(left, right) => Self::Sub(
                Box::new((*left).expand_fullpath(&env)),
                Box::new((*right).expand_fullpath(&env)),
            ),
            Self::Mul(left, right) => Self::Mul(
                Box::new((*left).expand_fullpath(&env)),
                Box::new((*right).expand_fullpath(&env)),
            ),
            Self::Div(left, right) => Self::Div(
                Box::new((*left).expand_fullpath(&env)),
                Box::new((*right).expand_fullpath(&env)),
            ),
            Self::Rem(left, right) => Self::Rem(
                Box::new((*left).expand_fullpath(&env)),
                Box::new((*right).expand_fullpath(&env)),
            ),
            Self::Exp(left, right) => Self::Exp(
                Box::new((*left).expand_fullpath(&env)),
                Box::new((*right).expand_fullpath(&env)),
            ),
            _ => todo!(),
        }
    }

    pub fn eval_to_vector(self, env: &Env) -> PqlVector {
        match self.to_owned() {
            Expr::Selector(_selector) => {
                todo!()
                // let path = path.expand_fullpath(&env);
                // let v = book
                //     .source_fields
                //     .get(path.to_string().as_str())
                //     .unwrap()
                //     .to_owned();
                // PqlVector(v)
            }
            Self::Num(_num) => {
                todo!()
                // PqlVector(vec![PqlValue::Float(OrderedFloat(num)); book.column_size]),
            }
            Self::Add(box expr1, box expr2) => {
                expr1.eval_to_vector(&env) + expr2.eval_to_vector(&env)
            }
            Self::Sub(box expr1, box expr2) => {
                expr1.eval_to_vector(&env) - expr2.eval_to_vector(&env)
            }
            Self::Mul(box expr1, box expr2) => {
                expr1.eval_to_vector(&env) * expr2.eval_to_vector(&env)
            }
            Self::Div(box expr1, box expr2) => {
                expr1.eval_to_vector(&env) / expr2.eval_to_vector(&env)
            }
            Self::Rem(box expr1, box expr2) => {
                expr1.eval_to_vector(&env) % expr2.eval_to_vector(&env)
            }
            _ => todo!(),
        }
    }

    pub fn eval(self) -> PqlValue {
        match self.to_owned() {
            Self::Value(value) => value,
            Self::Star => todo!(),
            Self::Selector(_) => todo!(),
            Self::Num(num) => PqlValue::Float(OrderedFloat(num.to_owned())),
            Self::Func(_) => todo!(),
            Self::Sql(_) => todo!(),
            Self::Add(box expr1, box expr2) => (expr1).eval() + (expr2).eval(),
            Self::Sub(box expr1, box expr2) => (expr1).eval() - (expr2).eval(),
            Self::Mul(box expr1, box expr2) => (expr1).eval() * (expr2).eval(),
            Self::Div(box expr1, box expr2) => (expr1).eval() / (expr2).eval(),
            Self::Rem(box expr1, box expr2) => (expr1).eval() % (expr2).eval(),
            Self::Exp(box expr1, box expr2) => (expr1).eval().powf((expr2).eval()),
        }
    }

    pub fn source_field_name_set(&self, env: &Env) -> HashSet<String> {
        match self.to_owned() {
            Expr::Num(_) => HashSet::default(),
            Expr::Selector(selector) => {
                collect! {
                    as HashSet<String>:
                    selector.expand_fullpath2(&env).to_string()
                }
            }
            Expr::Add(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&env);
                let b = expr2.source_field_name_set(&env);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Sub(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&env);
                let b = expr2.source_field_name_set(&env);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Mul(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&env);
                let b = expr2.source_field_name_set(&env);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Div(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&env);
                let b = expr2.source_field_name_set(&env);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Rem(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&env);
                let b = expr2.source_field_name_set(&env);
                a.union(&b).map(String::from).collect::<HashSet<_>>()
            }
            Expr::Exp(box expr1, box expr2) => {
                let a = expr1.source_field_name_set(&env);
                let b = expr2.source_field_name_set(&env);
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
