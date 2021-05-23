use std::collections::HashMap;

use crate::models::JsonValue;
use crate::pqlir_parser;
use crate::sql::to_list;
use crate::sql::DField;
use crate::sql::Dpath;
use crate::sql::Sql;
use crate::sql_parser;

#[derive(Debug, Clone, PartialEq)]
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

    fn rec_get_full_path(&self, path: &Dpath, trace_path: &mut Dpath) {
        if let Some((first, tail)) = path.to_vec().split_first() {
            if let Some(alias_path) = self.from_alias(first) {
                self.rec_get_full_path(&alias_path, trace_path)
            } else {
                (*trace_path).data.push(first.to_string());
            }
            if tail.len() > 0 {
                let tail_path = Dpath::from(tail);
                (*trace_path).data.push(tail_path.to_string());
            }
        }
    }

    pub fn get_full_path(&self, path: &Dpath) -> Dpath {
        let mut trace_path = Dpath::default();

        self.rec_get_full_path(path, &mut trace_path);
        trace_path
    }
}

#[cfg(test)]
mod tests {
    use super::Bingings;
    use crate::sql::{DField, DSql, Dpath};

    #[test]
    fn get_full_path() {
        let sql = DSql {
            select_clause: vec![
                DField {
                    path: Dpath::from("e.name"),
                    alias: Some("employeeName".to_owned()),
                },
                DField {
                    path: Dpath::from("p.name"),
                    alias: Some("projectName".to_owned()),
                },
            ],
            from_clause: vec![
                DField {
                    path: Dpath::from("hr.employeesNest"),
                    alias: Some("e".to_owned()),
                },
                DField {
                    path: Dpath::from("e.projects"),
                    alias: Some("p".to_owned()),
                },
            ],
            where_clause: None,
        };

        let bingings = Bingings::from(
            sql.select_clause
                .into_iter()
                .chain(sql.from_clause.into_iter())
                .collect::<Vec<_>>()
                .as_slice(),
        );

        let field = DField {
            path: Dpath::from("e.name"),
            alias: Some("employeetName".to_owned()),
        };
        assert_eq!(
            bingings.get_full_path(&field.path).to_string(),
            "hr.employeesNest.name",
        );

        let field = DField {
            path: Dpath::from("p.name"),
            alias: Some("projectName".to_owned()),
        };
        assert_eq!(
            bingings.get_full_path(&field.path).to_string(),
            "hr.employeesNest.projects.name",
        );
    }
}
