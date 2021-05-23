use std::collections::HashMap;

use itertools::Itertools;

use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::to_list;
use partiql::sql::DField;
use partiql::sql::Dpath;
use partiql::sql::Sql;
use partiql::sql_parser;

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

    let list = to_list(value);
    dbg!(&list);

    let ss = list
        .into_iter()
        .filter_map(|value| {
            let re = regex::Regex::new("security").unwrap();
            let path = Dpath::from(vec!["projectName"].as_slice());
            match value.select_by_path(path) {
                Some(JsonValue::Str(s)) if re.is_match(&s) => Some(value),
                _ => None,
            }
        })
        .collect::<Vec<JsonValue>>();
    dbg!(ss);

    dbg!("END OF FILE");
    Ok(())
}
