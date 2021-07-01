use std::str::FromStr;

use indexmap::IndexMap as Map;
use itertools::Itertools;

use partiql::parser;
use partiql::planner::Filter;
use partiql::sql::Env;
use partiql::sql::Field;
use partiql::value::PqlValue;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        PqlValue::from_str(&input)?
    };

    let condition =
        parser::clauses::parse_where(r#"WHERE hr.employeesNest.projects.name LIKE '%security%'"#)?
            .1;
    let data = Filter(Some(Box::new(condition))).execute(data, &Env::default());

    let fields = vec![
        Field::from_str("hr.employeesNest.projects.name AS projectName")?,
        Field::from_str("hr.employeesNest.name AS employeeName")?,
    ];
    let data = data.select_by_fields(&fields);
    println!("{}", data.clone().unwrap().to_json()?);

    dbg!(&data);

    let (tables, n, keys) = {
        let mut tables = Map::<String, Vec<PqlValue>>::new();
        let mut n = 0;
        let mut keys = vec![];
        if let Some(PqlValue::Object(map)) = data {
            keys = map
                .keys()
                .into_iter()
                .map(|s| s.to_string())
                .collect::<Vec<String>>();
            for (key, value) in map {
                if let PqlValue::Array(array) = value {
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
    // println!("{}", tables.to_owned().to_json()?);

    let records = {
        let mut records = Vec::<Map<String, Vec<PqlValue>>>::new();
        for i in 0..n {
            let mut record = Map::<String, Vec<PqlValue>>::new();
            for key in &keys {
                let v = tables.get(key.as_str()).unwrap().get(i).unwrap();
                // record.insert(key.to_string(), v.to_owned());
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
    dbg!(&records);
    // println!("{}", records.to_owned().to_json()?);

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
                    .collect::<Map<String, _>>();
                let v = PqlValue::Object(map);
                v
            })
            .collect::<Vec<PqlValue>>()
        })
        .flatten()
        .collect::<Vec<PqlValue>>();
    dbg!(list);

    Ok(())
}
