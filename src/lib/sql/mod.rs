use std::collections::HashMap;

use crate::models::JsonValue;

mod bingings;
mod utils;
pub use bingings::Bingings;
pub use utils::to_list;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub source: String,
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DField {
    pub source: String,
    pub path: Dpath,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sql {
    pub select_clause: Vec<Field>,
    pub from_clause: Vec<Field>,
    pub alias_map: HashMap<String, Field>,
    pub left_join_clause: Vec<Field>,
    pub where_clause: Option<WhereCond>,
}

impl Sql {
    fn rec_get_full_path(&self, field: &Field, path: &mut Vec<String>) {
        if let Some(alias_field) = self.alias_map.get(&field.source) {
            self.rec_get_full_path(alias_field, path);
        } else {
            (*path).push(field.source.clone());
        }
        (*path).push(field.path.clone());
    }

    pub fn get_full_path(&self, field: &Field) -> Vec<String> {
        let mut path = Vec::<String>::new();
        self.rec_get_full_path(field, &mut path);
        path
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhereCond {
    Eq { field: Field, right: String },
    Like { field: Field, right: String },
}

impl WhereCond {
    pub fn eval(&self, left: &JsonValue) -> bool {
        match self {
            Self::Eq { field, right } => {
                if let Some(value) = left.clone().get(&field.path) {
                    if value == JsonValue::Str(right.to_owned()) {
                        true
                    } else {
                        false
                    }
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
                // let r = match left.clone().get(&field.path) {
                let path = field.path.split(".").collect::<Vec<_>>();
                let val = left.by_path(&path);
                dbg!(&field, &path, &left, &val);
                match val {
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
