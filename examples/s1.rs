use std::collections::HashMap;

use itertools::Itertools;

use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::to_list;
use partiql::sql::DField;
use partiql::sql::Dpath;
use partiql::sql::Sql;
use partiql::sql::WhereCond;
use partiql::sql_parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let fields = vec![
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["hr", "employees", "id"].as_slice()),
            alias: None,
        },
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["hr", "employees", "title"].as_slice()),
            alias: Some("title".to_owned()),
        },
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["hr", "employees", "name"].as_slice()),
            alias: Some("employeeName".to_owned()),
        },
    ];

    let locals = fields
        .iter()
        .filter_map(|field| {
            if let Some(alias) = &field.alias {
                Some((alias.to_string(), field.path.to_owned()))
            } else {
                None
            }
        })
        .collect::<HashMap<String, Dpath>>();
    dbg!(&locals);

    let value = data.select_by_fields(&fields).unwrap();
    dbg!(&value);

    let list = to_list(value);
    dbg!(&list);

    let ss = list
        .into_iter()
        .filter_map(|value| {
            let s = value
                .select_by_path(Dpath::from(vec!["title"].as_slice()))
                .unwrap();
            if s == JsonValue::Str("Dev Mgr".to_owned()) {
                Some(value)
            } else {
                None
            }
        })
        .collect::<Vec<JsonValue>>();
    dbg!(ss);

    dbg!("END OF FILE");
    Ok(())
}
