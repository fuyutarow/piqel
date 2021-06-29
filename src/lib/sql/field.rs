use crate::sql::Bindings;
use crate::sql::Selector;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Field {
    pub path: Selector,
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
