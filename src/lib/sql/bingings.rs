use std::collections::HashMap;

use crate::models::JsonValue;
use crate::pqlir_parser;
use crate::sql::to_list;
use crate::sql::DField;
use crate::sql::Dpath;
use crate::sql::Sql;
use crate::sql_parser;

pub struct Bingings {
    locals: HashMap<String, Dpath>,
    locals_rev: HashMap<String, String>,
}

impl From<&[DField]> for Bingings {
    fn from(fields: &[DField]) -> Self {
        let locals = fields
            .iter()
            .filter_map(|field| {
                if let Some(alias) = &field.alias {
                    Some((alias.to_string(), field.path.to_owned()))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, Dpath>>();

        let locals_rev = fields
            .iter()
            .filter_map(|field| {
                if let Some(alias) = &field.alias {
                    Some((field.path.to_string(), alias.to_string()))
                } else {
                    None
                }
            })
            .collect::<HashMap<String, String>>();

        Self { locals, locals_rev }
    }
}

impl Bingings {
    pub fn to_alias(&self, path: &Dpath) -> Option<Dpath> {
        if let Some(alias) = self.locals_rev.get(&path.to_string()) {
            Some(Dpath::from(alias.as_str()))
        } else {
            None
        }
    }

    pub fn from_alias(&self, alias: &str) -> Option<Dpath> {
        self.locals.get(alias).map(|e| e.to_owned())
    }
}
