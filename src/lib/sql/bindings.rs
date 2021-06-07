use std::collections::VecDeque;

use indexmap::IndexMap as Map;

use crate::sql::DPath;
use crate::sql::Field;

#[derive(Debug, Clone, PartialEq)]
pub struct Bindings {
    locals: Map<String, DPath>,
    locals_rev: Map<String, String>,
}

impl From<&[Field]> for Bindings {
    fn from(fields: &[Field]) -> Self {
        let locals = fields
            .iter()
            .filter_map(|field| {
                if let Some(alias) = &field.alias {
                    Some((alias.to_string(), field.path.to_owned()))
                } else {
                    None
                }
            })
            .collect::<Map<String, DPath>>();

        let locals_rev = fields
            .iter()
            .filter_map(|field| {
                if let Some(alias) = &field.alias {
                    Some((field.path.to_string(), alias.to_string()))
                } else {
                    None
                }
            })
            .collect::<Map<String, String>>();

        Self { locals, locals_rev }
    }
}

impl Bindings {
    //     pub fn to_alias(&self, path: &Dpath) -> Option<Dpath> {
    //         if let Some(alias) = self.locals_rev.get(&path.to_string()) {
    //             Some(Dpath::from(alias.as_str()))
    //         } else {
    //             None
    //         }
    //     }

    pub fn from_alias(&self, alias: &str) -> Option<DPath> {
        self.locals.get(alias).map(|e| e.to_owned())
    }

    fn rec_get_full_path(&self, path: &DPath, trace_path: &mut DPath) {
        if let Some((first, tail)) = path.to_vec().split_first() {
            if let Some(alias_path) = self.from_alias(first) {
                self.rec_get_full_path(&alias_path, trace_path)
            } else {
                (*trace_path).data.push_back(first.to_string());
            }
            if tail.len() > 0 {
                let tail_path = DPath::from(tail);
                // for p in tail_path.to_vec()
                let mut vec_path = tail_path
                    .to_vec()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<VecDeque<_>>();
                (*trace_path).data.append(&mut vec_path);
            }
        }
    }

    pub fn get_full_path(&self, path: &DPath) -> DPath {
        let mut trace_path = DPath::default();

        self.rec_get_full_path(path, &mut trace_path);
        trace_path
    }
}

#[cfg(test)]
mod tests {
    use super::Bindings;
    use crate::parser;
    use crate::sql::{DPath, Field, Sql};

    #[test]
    fn get_full_path() -> anyhow::Result<()> {
        let sql = Sql {
            select_clause: parser::parse_select_clause(
                r#"SELECT e.name AS employeeName, p.name AS projectName"#,
            )?
            .1,
            from_clause: vec![
                Field {
                    path: DPath::from("hr.employeesNest"),
                    alias: Some("e".to_owned()),
                },
                Field {
                    path: DPath::from("e.projects"),
                    alias: Some("p".to_owned()),
                },
            ],
            left_join_clause: vec![],
            where_clause: None,
            limit: None,
        };

        let fields = sql
            .from_clause
            .iter()
            .chain(sql.left_join_clause.iter())
            .map(|e| e.to_owned())
            .collect::<Vec<_>>();

        let bindings = Bindings::from(fields.as_slice());

        let field = Field {
            path: DPath::from("e.name"),
            alias: Some("employeetName".to_owned()),
        };
        assert_eq!(
            bindings.get_full_path(&field.path).to_string(),
            "hr.employeesNest.name",
        );

        let field = Field {
            path: DPath::from("p.name"),
            alias: Some("projectName".to_owned()),
        };
        assert_eq!(
            bindings.get_full_path(&field.path).to_string(),
            "hr.employeesNest.projects.name",
        );

        Ok(())
    }
}
