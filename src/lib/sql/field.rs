use crate::sql::Bindings;
use crate::sql::Expr;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub path: DPath,
    pub alias: Option<String>,
}

impl Field {
    pub fn expand_fullpath(&self, bindings: &Bindings) -> Self {
        Self {
            path: self.path.expand_fullpath(&bindings),
            alias: self.alias.to_owned(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct DPath {
    pub data: Vec<String>,
}

impl From<&[&str]> for DPath {
    fn from(ss: &[&str]) -> Self {
        let data = ss.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        Self { data }
    }
}

impl From<&str> for DPath {
    fn from(s: &str) -> Self {
        let data = s
            .to_string()
            .split(".")
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        Self { data }
    }
}

impl DPath {
    pub fn to_string(&self) -> String {
        self.data.join(".")
    }

    pub fn to_vec(&self) -> Vec<&str> {
        self.data.iter().map(|s| s.as_str()).collect::<Vec<_>>()
    }

    pub fn expand_fullpath(&self, bidings: &Bindings) -> Self {
        // bidings.get_full_path(&self)
        todo!()
    }
}
