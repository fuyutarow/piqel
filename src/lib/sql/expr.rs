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

impl From<i64> for Expr {
    fn from(i: i64) -> Self {
        Self::Value(PqlValue::Int(i))
    }
}

impl From<f64> for Expr {
    fn from(f: f64) -> Self {
        Self::Value(PqlValue::Float(OrderedFloat(f)))
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
            Expr::Value(_) => self.to_owned(),
            Expr::Star => todo!(),
            Expr::Func(_) => todo!(),
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
            Expr::Sql(_) => todo!(),
        }
    }

    pub fn eval(self, env: &Env) -> PqlValue {
        match self.to_owned() {
            Self::Value(value) => value,
            Self::Selector(selector) => selector.evaluate(&env).unwrap_or_default(),
            Self::Star => todo!(),
            Self::Func(_) => todo!(),
            Self::Sql(_) => todo!(),
            Self::Add(box expr1, box expr2) => (expr1).eval(&env) + (expr2).eval(&env),
            Self::Sub(box expr1, box expr2) => (expr1).eval(&env) - (expr2).eval(&env),
            Self::Mul(box expr1, box expr2) => (expr1).eval(&env) * (expr2).eval(&env),
            Self::Div(box expr1, box expr2) => (expr1).eval(&env) / (expr2).eval(&env),
            Self::Rem(box expr1, box expr2) => (expr1).eval(&env) % (expr2).eval(&env),
            Self::Exp(box expr1, box expr2) => (expr1).eval(&env).powf((expr2).eval(&env)),
        }
    }

    pub fn source_field_name_set(&self, env: &Env) -> HashSet<String> {
        match self.to_owned() {
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::parser;
    use crate::planner::LogicalPlan;
    use crate::planner::Sql;
    use crate::sql::Env;
    use crate::value::PqlValue;

    #[test]
    fn test_expr_mul() -> anyhow::Result<()> {
        let mut sql = Sql::default();
        sql.select_clause = parser::clauses::select(r#"SELECT 4 * a AS aa"#)?.1;
        sql.from_clause = parser::clauses::from("FROM 3 as a")?.1;
        let plan = LogicalPlan::from(sql);

        let mut env = Env::default();
        let res = plan.execute(PqlValue::default(), &mut env);
        assert_eq!(res, PqlValue::from_str(r#"[{ "aa": 12 }]"#)?);

        Ok(())
    }
}
