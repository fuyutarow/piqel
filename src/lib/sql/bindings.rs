use std::collections::VecDeque;

use indexmap::IndexMap as Map;

use crate::sql::Field;
use crate::sql::Selector;
use crate::sql::SelectorNode;
use crate::sql::SourceValue;

#[derive(Debug, Clone, PartialEq)]
pub struct Bindings {
    locals: Map<String, Selector>,
}

impl From<&[Field]> for Bindings {
    fn from(fields: &[Field]) -> Self {
        let locals = fields
            .iter()
            .filter_map(|field| match field {
                Field {
                    value: SourceValue::Selector(selector),
                    alias: Some(alias),
                } => Some((alias.to_string(), selector.to_owned())),
                _ => todo!(),
            })
            .collect::<Map<String, Selector>>();

        Self { locals }
    }
}

impl Bindings {
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

    pub fn get_full_path(&self, value: &SourceValue) -> Selector {
        match &value {
            SourceValue::Selector(selector) => {
                let mut trace_path = Selector::default();

                self.rec_get_full_path(selector, &mut trace_path);
                trace_path
            }
            SourceValue::Value(_) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

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
                Field::from_str("hr.employeesNest AS e")?,
                Field::from_str("e.projects AS p")?,
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

        assert_eq!(
            bindings
                .get_full_path(&Field::from_str("e.name AS employeeName")?.value)
                .to_string(),
            "hr.employeesNest.name",
        );

        assert_eq!(
            bindings
                .get_full_path(&Field::from_str("p.name AS projectName")?.value)
                .to_string(),
            "hr.employeesNest.projects.name",
        );

        Ok(())
    }
}
