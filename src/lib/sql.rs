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
