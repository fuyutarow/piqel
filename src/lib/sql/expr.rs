use crate::sql::Bindings;
use crate::sql::DPath;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Path(DPath),
    Num(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Exp(Box<Expr>, Box<Expr>),
}

impl Expr {
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
            Self::Exp(left, right) => Self::Exp(
                Box::new((*left).expand_fullpath(&bindings)),
                Box::new((*right).expand_fullpath(&bindings)),
            ),
        }
    }

    pub fn eval(&self) -> f64 {
        match self {
            Self::Path(_) => 0.,
            Self::Num(num) => num.to_owned(),
            Expr::Add(expr1, expr2) => (*expr1).eval() + (*expr2).eval(),
            Expr::Sub(expr1, expr2) => (*expr1).eval() - (*expr2).eval(),
            Expr::Mul(expr1, expr2) => (*expr1).eval() * (*expr2).eval(),
            Expr::Div(expr1, expr2) => (*expr1).eval() / (*expr2).eval(),
            Expr::Exp(expr1, expr2) => (*expr1).eval().powf((*expr2).eval()),
        }
    }
}