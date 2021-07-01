use std::collections::VecDeque;

use indexmap::IndexMap as Map;

use crate::sql::Selector;
use crate::sql::SelectorNode;
use crate::sql::SourceValue;
use crate::value::PqlValue;

#[derive(Debug, Default, Clone)]
pub struct Env {
    data: Map<String, SourceValue>,
}

impl Env {
    pub fn insert(&mut self, alias: &str, value: &SourceValue) -> Option<SourceValue> {
        self.data.insert(alias.to_string(), value.to_owned())
    }

    pub fn insert_from_selector(
        &mut self,
        alias: &str,
        selector: &Selector,
    ) -> Option<SourceValue> {
        let value = SourceValue::Selector(selector.to_owned());
        self.insert(alias, &value)
    }

    pub fn insert_from_pqlval(&mut self, alias: &str, value: &PqlValue) -> Option<SourceValue> {
        let value = SourceValue::Value(value.to_owned());
        self.insert(alias, &value)
    }

    pub fn get(&self, key: &str) -> Option<SourceValue> {
        self.data.get(key).map(|e| e.to_owned())
    }

    pub fn get_as_selector(&self, key: &str) -> Option<Selector> {
        match self.get(key) {
            Some(SourceValue::Selector(selector)) => Some(selector),
            _ => None,
        }
    }

    fn rec_get_full_path(&self, selector: &Selector, trace_path: &mut Selector) {
        if let Some((first, tail)) = selector.split_first() {
            if let Some(alias_path) = self.get_as_selector(&first.to_string()) {
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

    pub fn expand_fullpath_as_selector(&self, selector: &Selector) -> Selector {
        let mut trace_path = Selector::default();

        self.rec_get_full_path(selector, &mut trace_path);
        trace_path
    }

    pub fn expand_fullpath(&self, value: &SourceValue) -> SourceValue {
        match &value {
            SourceValue::Selector(selector) => {
                let mut trace_path = Selector::default();

                self.rec_get_full_path(selector, &mut trace_path);
                SourceValue::Selector(trace_path)
            }
            SourceValue::Value(_) => value.to_owned(),
            _ => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Env;
    use crate::planner::Drain;
    use crate::planner::Sql;
    use crate::sql::Field;

    #[test]
    fn get_full_path() -> anyhow::Result<()> {
        let sql = Sql::from_str(
            r#"
SELECT
  e.name AS employeeName, p.name AS projectName
FROM
  hr.employeesNest AS e, e.projects AS p
        "#,
        )?;

        let mut env = Env::default();
        Drain(sql.from_clause).excute(&mut env);

        assert_eq!(
            env.expand_fullpath(&Field::from_str("e.name AS employeeName")?.value)
                .to_string(),
            "hr.employeesNest.name",
        );

        assert_eq!(
            env.expand_fullpath(&Field::from_str("p.name AS projectName")?.value)
                .to_string(),
            "hr.employeesNest.projects.name",
        );

        Ok(())
    }
}
