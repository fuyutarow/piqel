use std::collections::HashMap;

use itertools::Itertools;

use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::to_list;
use partiql::sql::Bingings;
use partiql::sql::DField;
use partiql::sql::DSql;
use partiql::sql::Dpath;
use partiql::sql_parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let sql = {
        let input = std::fs::read_to_string("samples/q2.sql").unwrap();
        let sql = sql_parser::sql(&input)?;
        sql
    };
    // dbg!(&sql);

    let sql = DSql {
        select_clause: vec![
            DField {
                path: Dpath::from("e.name"),
                alias: Some("employeeName".to_owned()),
            },
            DField {
                path: Dpath::from("p.name"),
                alias: Some("projectName".to_owned()),
            },
        ],
        from_clause: vec![
            DField {
                path: Dpath::from("hr.employeesNest"),
                alias: Some("e".to_owned()),
            },
            DField {
                path: Dpath::from("e.projects"),
                alias: Some("p".to_owned()),
            },
        ],
    };
    dbg!(&sql);

    let env = Bingings::from(sql.from_clause.as_slice());
    dbg!(&env);

    let env = Bingings::from(
        sql.select_clause
            .into_iter()
            .chain(sql.from_clause.into_iter())
            .collect::<Vec<_>>()
            .as_slice(),
    );
    dbg!(&env);

    let field = DField {
        path: Dpath::from("p.name"),
        alias: Some("projectName".to_owned()),
    };
    let p = env.get_full_path(&field.path);
    dbg!(&p);

    let field = DField {
        path: Dpath::from("e.name"),
        alias: Some("employeeName".to_owned()),
    };

    let p = env.get_full_path(&field.path);
    dbg!(&p);

    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let fields = vec![
        DField {
            path: Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice()),
            alias: Some("projectName".to_owned()),
        },
        DField {
            path: Dpath::from(vec!["hr", "employeesNest", "name"].as_slice()),
            alias: Some("employeeName".to_owned()),
        },
    ];

    let value = data.select_by_fields(&fields).unwrap();

    let list = to_list(value);

    let bingins = Bingings::from(fields.as_slice());

    let ss = list
        .into_iter()
        .filter_map(|value| {
            let re = regex::Regex::new("security").unwrap();
            let path = Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice());
            let access_path = bingins.to_alias(&path).unwrap_or(path);
            match value.select_by_path(access_path) {
                Some(JsonValue::Str(s)) if re.is_match(&s) => Some(value),
                _ => None,
            }
        })
        .collect::<Vec<JsonValue>>();
    dbg!(ss);

    dbg!("END OF FILE");
    Ok(())
}
