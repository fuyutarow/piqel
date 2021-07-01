use indexmap::IndexMap as Map;
use itertools::Itertools;

use crate::parser;
use crate::pqlir_parser;
use crate::sql::restrict;
use crate::sql::Env;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::FieldBook;
use crate::sql::Proj;
use crate::sql::WhereCond;
use crate::value::BPqlValue;
use crate::value::PqlValue;

#[derive(Debug, Default)]
pub struct LogicalPlan {
    pub drains: Vec<Drain>,
    pub filter: Filter,
    pub project: Projection,
}

impl LogicalPlan {}

#[derive(Debug, Default, Clone)]
pub struct Drain(pub Vec<Field>);

impl Drain {
    pub fn excute(self, env: &mut Env) {
        for field in self.0 {
            if let Some(alias) = field.alias {
                env.insert(&alias, &field.value);
            }
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Filter(pub Option<Box<WhereCond>>);

impl Filter {
    pub fn execute(self, data: PqlValue, env: &Env) -> PqlValue {
        match &self.0 {
            Some(box WhereCond::Eq { expr, right }) => match expr {
                Expr::Path(selector) => {
                    let selector = selector.expand_fullpath2(&env);
                    let cond = WhereCond::Eq {
                        expr: expr.to_owned(),
                        right: right.to_owned(),
                    };
                    restrict(Some(data.to_owned()), &selector, &Some(cond))
                        .expect("restricted value")
                }
                _ => {
                    todo!();
                }
            },
            Some(box WhereCond::Like { expr, right }) => match expr {
                Expr::Path(selector) => {
                    let path = selector.expand_fullpath2(&env);
                    let cond = WhereCond::Like {
                        expr: expr.to_owned(),
                        right: right.to_owned(),
                    };
                    restrict(Some(data.to_owned()), &path, &Some(cond)).expect("restricted value")
                }
                _ => {
                    todo!();
                }
            },
            _ => todo!(),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Projection(pub Vec<Field>);

impl Projection {
    pub fn execute(self, data: PqlValue) -> Vec<PqlValue> {
        let projected = data.select_by_fields(&self.0).unwrap_or_default();
        Records::from(projected).into_list()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Records(pub Map<String, Vec<PqlValue>>);

impl From<PqlValue> for Records {
    fn from(value: PqlValue) -> Self {
        Self(match value {
            PqlValue::Object(record) => record
                .into_iter()
                .map(|(k, v)| match v {
                    PqlValue::Array(array) => (k, array),
                    _ => unreachable!(),
                })
                .collect::<Map<String, Vec<PqlValue>>>(),
            _ => unreachable!(),
        })
    }
}

impl Records {
    fn into_list(self) -> Vec<PqlValue> {
        let keys = self.0.keys();
        let it = self.0.values().into_iter().multi_cartesian_product();
        it.map(|prod| {
            let map = keys
                .clone()
                .into_iter()
                .zip(prod.into_iter())
                .map(|(key, p)| (key.to_owned(), p.to_owned()))
                .collect::<Map<String, _>>();
            let v = PqlValue::Object(map);
            v
        })
        .collect::<Vec<PqlValue>>()
    }
}
