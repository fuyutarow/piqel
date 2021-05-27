use crate::value::PqlValue;

mod bindings;
mod eval;
pub mod parser;
mod utils;
pub use bindings::Bindings;
pub use eval::{run, to_list};

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Path(Dpath),
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub expr: Expr,
    pub alias: Option<String>,
}

impl Field {
    pub fn expand_fullpath(&self, bindings: &Bindings) -> Self {
        Self {
            expr: self.expr.expand_fullpath(&bindings),
            alias: self.alias.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DField {
    pub path: Dpath,
    pub alias: Option<String>,
}

impl DField {
    pub fn full(&self, bidings: &Bindings) -> Self {
        let path = bidings.get_full_path(&self.path);
        Self {
            path,
            alias: self.alias.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Dpath {
    pub data: Vec<String>,
}

impl From<&[&str]> for Dpath {
    fn from(ss: &[&str]) -> Self {
        let data = ss.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Self { data }
    }
}

impl From<&str> for Dpath {
    fn from(s: &str) -> Self {
        let data = s
            .to_string()
            .split(".")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Self { data }
    }
}

impl Dpath {
    pub fn to_string(&self) -> String {
        self.data.join(".")
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.data.iter().map(|s| s.as_str()).collect::<Vec<_>>()
    }

    pub fn expand_fullpath(&self, bidings: &Bindings) -> Self {
        bidings.get_full_path(&self)
    }

    pub fn full(&self, bidings: &Bindings) -> Self {
        bidings.get_full_path(&self)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DSql {
    pub select_clause: Vec<DField>,
    pub from_clause: Vec<DField>,
    pub left_join_clause: Vec<DField>,
    pub where_clause: Option<DWhereCond>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DWhereCond {
    Eq { field: DField, right: String },
    Like { field: DField, right: String },
}

impl DWhereCond {
    pub fn eval(
        &self,
        left: &PqlValue,
        bindings: &Bindings,
        bindings_for_select: &Bindings,
    ) -> bool {
        match self {
            Self::Eq { field, right } => {
                let where_arg_path = field.path.full(&bindings);
                let access_path = bindings_for_select
                    .to_alias(&where_arg_path)
                    .unwrap_or(where_arg_path.to_owned());
                if let Some(value) = left.clone().select_by_path(&access_path) {
                    value == PqlValue::Str(right.to_owned())
                } else {
                    false
                }
            }
            Self::Like { field, right } => {
                let pattern = match (right.starts_with("%"), right.ends_with("%")) {
                    (true, true) => {
                        format!("{}", right.trim_start_matches("%").trim_end_matches("%"))
                    }
                    (true, false) => format!("{}$", right.trim_start_matches("%")),
                    (false, true) => format!("^{}", right.trim_end_matches("%")),
                    (false, false) => format!("^{}$", right),
                };
                let re = regex::Regex::new(&pattern).unwrap();

                let where_arg_path = field.path.full(&bindings);
                let access_path = bindings_for_select
                    .to_alias(&where_arg_path)
                    .unwrap_or(where_arg_path.to_owned());
                match left.select_by_path(&access_path) {
                    Some(PqlValue::Str(s)) if re.is_match(&s) => true,
                    _ => false,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Dpath;

    #[test]
    fn dpath_from_vec() {
        let path = Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice());
        assert_eq!(path.to_string().as_str(), "hr.employeesNest.projects.name",);
        assert_eq!(
            path.to_vec(),
            vec!["hr", "employeesNest", "projects", "name"]
        );
    }

    #[test]
    fn dpath_from_str() {
        let path = Dpath::from("hr.employeesNest.projects.name");
        assert_eq!(path.to_string().as_str(), "hr.employeesNest.projects.name",);
        assert_eq!(
            path.to_vec(),
            vec!["hr", "employeesNest", "projects", "name"]
        );
    }
}
