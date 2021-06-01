use indexmap::IndexMap as Map;
use itertools::Itertools;
use rayon::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::sql::restrict;
use crate::sql::Bindings;
use crate::sql::Expr;
use crate::sql::Field;
use crate::sql::Sql;
use crate::sql::WhereCond;
use crate::value::PqlValue;

pub fn evaluate<'a>(sql: &Sql, data: &'a PqlValue) -> PqlValue {
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

    let select_fields = sql
        .select_clause
        .to_owned()
        .into_iter()
        .map(|proj| proj.to_field(&bindings))
        .collect::<Vec<Field>>();

    let d = data.select_by_fields(&select_fields).unwrap();
    let d = to_list(d);

    PqlValue::Array(d)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FieldBook {
    pub fields: Map<String, Vec<PqlValue>>,
    pub size: usize,
    pub keys: Vec<String>,
}

impl From<PqlValue> for FieldBook {
    fn from(pqlv_object: PqlValue) -> Self {
        let (fields, size, keys) = {
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

        Self { fields, size, keys }
    }
}

pub fn to_list(value_selected_by_fields: PqlValue) -> Vec<PqlValue> {
    let (tables, n, keys) = {
        let mut tables = Map::<String, Vec<PqlValue>>::new();
        let mut n = 0;
        let mut keys = vec![];
        if let PqlValue::Object(map) = value_selected_by_fields {
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

    let records = {
        let mut records = Vec::<Map<String, Vec<PqlValue>>>::new();
        for i in 0..n {
            let mut record = Map::<String, Vec<PqlValue>>::new();
            for key in &keys {
                let v = tables.get(key.as_str()).unwrap().get(i).unwrap();
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

    let list = records
        .into_par_iter()
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

    list
}
