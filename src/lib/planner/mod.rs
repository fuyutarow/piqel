mod drain;
mod filter;
pub mod project;

use std::str::FromStr;

use crate::parser;
pub use crate::sql::clause::Limit;
pub use crate::sql::clause::OrderBy;
use crate::sql::Env;
use crate::sql::Field;
pub use crate::sql::WhereCond;
use crate::value::BPqlValue;
use crate::value::PqlValue;
pub use drain::Drain;
pub use filter::Filter;
pub use project::Projection;

#[derive(Debug, Default)]
pub struct LogicalPlan {
    pub drains: Vec<Drain>,
    pub filter: Filter,
    pub project: Projection,
    pub order_by: Option<OrderBy>,
    pub limit: Option<Limit>,
}

impl From<Sql> for LogicalPlan {
    fn from(sql: Sql) -> Self {
        dbg!(&sql);
        Self {
            drains: vec![Drain(sql.from_clause), Drain(sql.left_join_clause)],
            filter: Filter(sql.where_clause),
            project: Projection(sql.select_clause),
            order_by: None,
            limit: None,
        }
    }
}

impl LogicalPlan {
    pub fn excute(self, data: PqlValue, env: &mut Env) -> PqlValue {
        for drain in self.drains {
            drain.excute(env);
        }
        dbg!(&data);
        let data = self.filter.execute(data, &env);
        dbg!(&data);
        let mut list = self.project.execute(data, &env);

        if let Some(orderby) = &self.order_by {
            let mut list_with_key = list
                .into_iter()
                .filter_map(|record| {
                    record
                        .to_owned()
                        .get(&orderby.label)
                        .map(|value| (BPqlValue::from(value), record))
                })
                .collect::<Vec<_>>();
            list_with_key.sort_by(|x, y| {
                if orderby.is_asc {
                    x.0.partial_cmp(&y.0).unwrap()
                } else {
                    y.0.partial_cmp(&x.0).unwrap()
                }
            });
            list = list_with_key
                .into_iter()
                .map(|(_k, v)| v)
                .collect::<Vec<_>>();
        }

        if let Some(limit_clause) = &self.limit {
            let (_, values) = list.split_at(limit_clause.offset as usize);
            let (values, _) = values.split_at(limit_clause.limit as usize);
            list = values.to_owned();
        }

        PqlValue::Array(list)
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Sql {
    pub select_clause: Vec<Field>,
    pub from_clause: Vec<Field>,
    pub left_join_clause: Vec<Field>,
    pub where_clause: Option<Box<WhereCond>>,
    pub orderby: Option<OrderBy>,
    pub limit: Option<Limit>,
}

impl FromStr for Sql {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        parser::select_statement::from_str(s)
    }
}

pub fn evaluate<'a>(sql: Sql, data: PqlValue) -> PqlValue {
    let mut env = Env::default();
    let plan = LogicalPlan::from(sql);
    let result = plan.excute(data, &mut env);
    result
}
