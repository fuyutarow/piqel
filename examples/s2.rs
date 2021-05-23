use std::collections::HashMap;

use itertools::Itertools;

use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::DField;
use partiql::sql::Dpath;
use partiql::sql::Sql;
use partiql::sql_parser;

fn transpose_records<T: Clone>(records: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut transposed: Vec<Vec<T>> = vec![Vec::new(); records[0].len()];

    for record in records {
        for (index, element) in record.iter().enumerate() {
            transposed[index].push(element.clone());
        }
    }

    transposed
}

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    dbg!(data.select_by_fields(&[
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["hr", "employees", "name",].as_slice()),
            alias: Some("employeeName".to_owned()),
        },
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["hr", "employees", "id",].as_slice()),
            alias: Some("id".to_owned()),
        }
    ]));
    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    dbg!(data.select_by_fields(&[
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice()),
            alias: Some("projectName".to_owned()),
        },
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["hr", "employeesNest", "name"].as_slice()),
            alias: Some("employeeName".to_owned()),
        }
    ]));

    let value = data
        .select_by_fields(&[
            DField {
                source: "".to_owned(),
                path: Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice()),
                alias: Some("projectName".to_owned()),
            },
            DField {
                source: "".to_owned(),
                path: Dpath::from(vec!["hr", "employeesNest", "name"].as_slice()),
                alias: Some("employeeName".to_owned()),
            },
        ])
        .unwrap();

    dbg!(&value);

    let mut tables = HashMap::<String, Vec<JsonValue>>::new();

    let mut n = 0;
    let mut keys = vec![];
    if let JsonValue::Object(map) = value {
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

    dbg!(n);
    dbg!(&tables);

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
    dbg!(&records);
    // if let JsonValue::Object(map) = value {
    //     let keys = map
    //         .keys()
    //         .into_iter()
    //         .map(|s| s.to_string())
    //         .collect::<Vec<String>>();
    //     dbg!(keys.first());

    //     if let Some(JsonValue::Array(array)) = map.get(keys.first().unwrap()) {
    //         let n = array.len();

    //         for i in 0..n {
    //             let mut record = HashMap::new();

    //             for key in keys {
    //                 let value = map.get(key).unwrap()
    //             record.insert(key, )

    //             }

    //             records.push(record);
    //         }
    //     }

    //     // map.into_iter().collect
    //     // let records = transpose_records(tables);
    //     // dbg!(records);
    // }

    // for record in records {
    //     record.map()
    // }

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
    dbg!(list);

    dbg!("END OF FILE");
    Ok(())
}
