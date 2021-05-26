use indexmap::IndexMap;

use itertools::Itertools;

use partiql::pqlir_parser;
use partiql::sql::DField;
use partiql::sql::Dpath;
use partiql::value::JsonValue;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let value = data
        .select_by_fields(&[
            DField {
                path: Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice()),
                alias: Some("projectName".to_owned()),
            },
            DField {
                path: Dpath::from(vec!["hr", "employeesNest", "name"].as_slice()),
                alias: Some("employeeName".to_owned()),
            },
        ])
        .unwrap();
    dbg!(&value);

    let (tables, n, keys) = {
        let mut tables = IndexMap::<String, Vec<JsonValue>>::new();
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
        (tables, n, keys)
    };
    dbg!(&tables);

    let records = {
        let mut records = Vec::<IndexMap<String, Vec<JsonValue>>>::new();
        for i in 0..n {
            let mut record = IndexMap::<String, Vec<JsonValue>>::new();
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
    dbg!(&records);

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
                    .collect::<IndexMap<String, _>>();
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
