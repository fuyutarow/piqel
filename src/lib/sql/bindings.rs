use std::collections::VecDeque;

use indexmap::IndexMap as Map;

use crate::sql::Field;
use crate::sql::Selector;
use crate::sql::SelectorNode;

#[derive(Debug, Clone, PartialEq)]
pub struct Bindings {
    locals: Map<String, Selector>,
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
            .collect::<Map<String, Selector>>();

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

    pub fn from_alias(&self, alias: &str) -> Option<Selector> {
        self.locals.get(alias).map(|e| e.to_owned())
    }

    fn rec_get_full_path(&self, selector: &Selector, trace_path: &mut Selector) {
        if let Some((first, tail)) = selector.split_first() {
            if let Some(alias_path) = self.from_alias(&first.to_string()) {
                self.rec_get_full_path(&alias_path, trace_path)
            } else {
                (*trace_path)
                    .data
                    .push_back(SelectorNode::String(first.to_string()));
            }
            if tail.data.len() > 0 {
                let tail_path = Selector::from(tail);
                // for p in tail_path.to_vec()
                let mut vec_path = tail_path
                    .to_vec()
                    .into_iter()
                    .map(|s| SelectorNode::String(s.to_string()))
                    .collect::<VecDeque<_>>();
                (*trace_path).data.append(&mut vec_path);
            }
        }
    }

    pub fn get_full_path(&self, path: &Selector) -> Selector {
        let mut trace_path = Selector::default();

        self.rec_get_full_path(path, &mut trace_path);
        trace_path
    }
}

#[cfg(test)]
mod tests {
    use super::Bindings;
    use crate::parser;
    use crate::sql::{Field, Selector, Sql};

    #[test]
    fn get_full_path() -> anyhow::Result<()> {
        let sql = Sql {
            select_clause: parser::clauses::select(
                r#"SELECT e.name AS employeeName, p.name AS projectName"#,
            )?
            .1,
            from_clause: vec![
                Field {
                    path: Selector::from("hr.employeesNest"),
                    alias: Some("e".to_owned()),
                },
                Field {
                    path: Selector::from("e.projects"),
                    alias: Some("p".to_owned()),
                },
            ],
            left_join_clause: vec![],
            where_clause: None,
            orderby: None,
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
            path: Selector::from("e.name"),
            alias: Some("employeetName".to_owned()),
        };
        assert_eq!(
            bindings.get_full_path(&field.path).to_string(),
            "hr.employeesNest.name",
        );

        let field = Field {
            path: Selector::from("p.name"),
            alias: Some("projectName".to_owned()),
        };
        assert_eq!(
            bindings.get_full_path(&field.path).to_string(),
            "hr.employeesNest.projects.name",
        );

        Ok(())
    }
}
