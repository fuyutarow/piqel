use std::collections::HashMap;

use itertools::Itertools;

use crate::models::JsonValue;
use crate::pqlir_parser;
use crate::sql::Bingings;
use crate::sql::DField;
use crate::sql::Dpath;
use crate::sql::Sql;
use crate::sql_parser;

pub fn to_list(value_selected_by_fields: JsonValue) -> Vec<JsonValue> {
    let (tables, n, keys) = {
        let mut tables = HashMap::<String, Vec<JsonValue>>::new();
        let mut n = 0;
        let mut keys = vec![];
        if let JsonValue::Object(map) = value_selected_by_fields {
            keys = map
                .keys()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for (key, value) in map {
                if let JsonValue::Array(array) = value {
                    if n == 0 {
                        n = array.len();
                    }
                    tables.insert(key, array);
                }
            }
        }
        (tables, n, keys)
    };

    let records = {
        let mut records = Vec::<HashMap<String, Vec<JsonValue>>>::new();
        for i in 0..n {
            let mut record = HashMap::<String, Vec<JsonValue>>::new();
            for key in &keys {
                let v = tables.get(key.as_str()).unwrap().get(i).unwrap();
                // record.insert(key.to_string(), v.to_owned());
                match v {
                    JsonValue::Array(array) => {
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
        .into_iter()
        .map(|record| {
            let keys = record.keys();
            let it = record.values().into_iter().multi_cartesian_product();
            it.map(|prod| {
                let map = keys
                    .clone()
                    .into_iter()
                    .zip(prod.into_iter())
                    .map(|(key, p)| (key.to_owned(), p.to_owned()))
                    .collect::<HashMap<String, _>>();
                let v = JsonValue::Object(map);
                v
            })
            .collect::<Vec<JsonValue>>()
        })
        .flatten()
        .collect::<Vec<JsonValue>>();

    list
}
