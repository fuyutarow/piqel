use crate::sql::field::Field;
use crate::sql::Expr;
use crate::sql::Selector;
use crate::value::PqlValue;

#[derive(Debug, Clone, PartialEq)]
pub enum WhereCond {
    Eq { expr: Expr, right: PqlValue },
    Like { expr: Expr, right: String },
}

impl Default for WhereCond {
    fn default() -> Self {
        Self::Eq {
            expr: Expr::default(),
            right: PqlValue::default(),
        }
    }
}

impl WhereCond {
    pub fn get_expr(&self) -> Expr {
        match &self {
            Self::Eq { expr, right: _ } => expr.to_owned(),
            Self::Like { expr, right: _ } => expr.to_owned(),
        }
    }
}

pub fn re_from_str(pattern: &str) -> regex::Regex {
    let regex_pattern = match (pattern.starts_with("%"), pattern.ends_with("%")) {
        (true, true) => {
            format!("{}", pattern.trim_start_matches("%").trim_end_matches("%"))
        }
        (true, false) => format!("{}$", pattern.trim_start_matches("%")),
        (false, true) => format!("^{}", pattern.trim_end_matches("%")),
        (false, false) => format!("^{}$", pattern),
    };
    let re = regex::Regex::new(&regex_pattern).unwrap();
    re
}
