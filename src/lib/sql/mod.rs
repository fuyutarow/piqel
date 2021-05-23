use std::collections::HashMap;

use crate::models::JsonValue;

mod bingings;
mod eval;
mod utils;
pub use bingings::Bingings;
pub use eval::run;
pub use utils::to_list;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub source: String,
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DField {
    pub path: Dpath,
    pub alias: Option<String>,
}

impl DField {
    pub fn full(&self, bidings: &Bingings) -> Self {
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

    pub fn full(&self, bidings: &Bingings) -> Self {
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
        left: &JsonValue,
        bindings: &Bingings,
        bindings_for_select: &Bingings,
    ) -> bool {
        match self {
            Self::Eq { field, right } => {
                let where_arg_path = field.path.full(&bindings);
                let access_path = bindings_for_select
                    .to_alias(&where_arg_path)
                    .unwrap_or(where_arg_path.to_owned());
                if let Some(value) = left.clone().select_by_path(&access_path) {
                    value == JsonValue::Str(right.to_owned())
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
                    Some(JsonValue::Str(s)) if re.is_match(&s) => true,
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
    fn dpath() {
        let path = Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice());
        assert_eq!(path.to_string().as_str(), "hr.employeesNest.projects.name",);
        assert_eq!(
            path.to_vec(),
            vec!["hr", "employeesNest", "projects", "name"]
        );
    }
}
