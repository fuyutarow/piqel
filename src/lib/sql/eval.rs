use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;
use indexmap::IndexMap as Map;
use itertools::Itertools;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::parser;
use crate::sql::restrict;
use crate::sql::Bindings;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Proj;
use crate::sql::Sql;
use crate::sql::WhereCond;
use crate::value::PqlValue;

pub fn evaluate<'a>(sql: &Sql, data: &'a PqlValue) -> anyhow::Result<PqlValue> {
    let fields = sql
        .from_clause
        .iter()
        .chain(sql.left_join_clause.iter())
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();

    let bindings = Bindings::from(fields.as_slice());

    let data = match &sql.where_clause {
        None => data.to_owned(),
        Some(box WhereCond::Eq { expr, right }) => match expr {
            Expr::Path(path) => {
                let path = path.expand_fullpath(&bindings);
                let cond = WhereCond::Eq {
                    expr: expr.to_owned(),
                    right: right.to_owned(),
                };
                restrict(Some(data.to_owned()), &path, &Some(cond)).expect("restricted value")
            }
            _ => todo!(),
        },
        Some(box WhereCond::Like { expr, right }) => match expr {
            Expr::Path(path) => {
                let path = path.expand_fullpath(&bindings);
                let cond = WhereCond::Like {
                    expr: expr.to_owned(),
                    right: right.to_owned(),
                };
                restrict(Some(data.to_owned()), &path, &Some(cond)).expect("restricted value")
            }
            _ => todo!(),
        },
        Some(_) => todo!(),
    };

    let projs = sql
        .select_clause
        .to_owned()
        .into_iter()
        .map(|proj| Proj {
            expr: proj.expr.to_owned(),
            alias: Some(proj.target_field_name()),
        })
        .collect::<Vec<_>>();
    dbg!(&projs);

    let selected_source = {
        let v = projs
            .iter()
            .map(|proj| proj.source_field_name_set())
            .fold(HashSet::default(), |acc, x| {
                acc.union(&x).map(String::from).collect::<HashSet<_>>()
            });

        data.select_by_fields(
            v.into_iter()
                .map(|s| parser::parse_field(&s).unwrap().1)
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .unwrap()
    };

    let mut book = FieldBook::from(selected_source);
    book.project_fields(&projs);

    let records = book.to_record();
    let list = records.into_pqlv();

    Ok(list)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldBook {
    pub source_fields: Map<String, Vec<PqlValue>>,
    pub target_fields: Map<String, Vec<PqlValue>>,
    pub column_size: usize,
    pub source_keys: Vec<String>,
    pub target_keys: Vec<String>,
}

impl From<PqlValue> for FieldBook {
    fn from(pqlv_object: PqlValue) -> Self {
        let (source_fields, size, source_keys) = {
            let mut tables = Map::<String, Vec<PqlValue>>::new();
            let mut n = 0;
            let mut keys = vec![];
            if let PqlValue::Object(map) = pqlv_object {
                keys = map
                    .keys()
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>();
                for (key, value) in map {
                    match value {
                        PqlValue::Array(array) => {
                            if n == 0 {
                                n = array.len();
                            }
                            tables.insert(key, array);
                        }
                        _ => {
                            n = 1;
                            tables.insert(key, vec![value]);
                        }
                    }
                }
            }
            (tables, n, keys)
        };

        Self {
            column_size: size,
            source_keys,
            source_fields,
            target_keys: Vec::default(),
            target_fields: Map::default(),
        }
    }
}

impl FieldBook {
    pub fn project_fields(&mut self, projs: &[Proj]) {
        projs.iter().for_each(|proj| {
            let target_name = proj.to_owned().alias.unwrap();
            let target_field = proj.to_owned().eval(&self);
            self.target_keys.push(target_name.to_owned());
            self.target_fields.insert(target_name, target_field.0);
        });
    }

    pub fn to_record(&self) -> Records {
        let records = {
            let mut records = Vec::<Map<String, Vec<PqlValue>>>::new();
            for i in 0..self.column_size {
                let mut record = Map::<String, Vec<PqlValue>>::new();
                for key in &self.target_keys {
                    let v = self
                        .target_fields
                        .get(key.as_str())
                        .unwrap()
                        .get(i)
                        .unwrap();
                    match v {
                        PqlValue::Array(array) => {
                            record.insert(key.to_string(), array.to_owned());
                        }
                        _ => {
                            record.insert(key.to_string(), vec![v.to_owned()]);
                        }
                    }
                }
                records.push(record);
            }
            records
        };
        Records(records)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Records(pub Vec<Map<String, Vec<PqlValue>>>);

impl Records {
    pub fn into_pqlv(self) -> PqlValue {
        let list = self
            .0
            .into_iter()
            .map(|record| {
                let record = record
                    .into_iter()
                    .filter_map(|(k, v)| if !v.is_empty() { Some((k, v)) } else { None })
                    .collect::<Map<String, Vec<PqlValue>>>();

                let keys = record.keys();
                let it = record.values().into_iter().multi_cartesian_product();
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
            })
            .flatten()
            .collect::<Vec<PqlValue>>();
        PqlValue::Array(list)
    }
}
