use std::collections::HashMap;

use crate::models::JsonValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub source: String,
    pub path: String,
    pub alias: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sql {
    pub select_clause: Vec<Field>,
    pub from_clause: Vec<Field>,
    pub alias_map: HashMap<String, Field>,
    pub left_join_clause: Vec<Field>,
    pub where_clause: Option<WhereCond>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WhereCond {
    Eq { field: Field, right: String },
    Like { field: Field, right: String },
}

impl Sql {
    pub fn get_full_path(&self, field: Field, path: &mut Vec<String>) {
        if let Some(alias_field) = self.alias_map.get(&field.source) {
            self.get_full_path(alias_field.clone(), path);
        } else {
            (*path).push(field.source);
        }
        (*path).push(field.path);
    }
}
