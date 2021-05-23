use std::collections::HashMap;

use itertools::Itertools;

use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::to_list;
use partiql::sql::Bingings;
use partiql::sql::DField;
use partiql::sql::DSql;
use partiql::sql::DWhereCond;
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
        where_clause: Some(DWhereCond::Like {
            field: DField {
                path: Dpath::from("p.name"),
                alias: None,
            },
            right: "%security%".to_owned(),
        }),
    };
    dbg!(&sql);

    let fields = sql
        .select_clause
        .iter()
        .chain(sql.from_clause.iter())
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();
    let bindings = Bingings::from(fields.as_slice());

    let field = DField {
        path: Dpath::from("p.name"),
        alias: Some("projectName".to_owned()),
    };

    let p = field.path.full(&bindings);
    dbg!(&p);

    let field = DField {
        path: Dpath::from("e.name"),
        alias: Some("employeeName".to_owned()),
    };

    let p = field.path.full(&bindings);
    dbg!(&p);

    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let select_fields = sql
        .select_clause
        .iter()
        .map(|field| field.to_owned().full(&bindings))
        .collect::<Vec<_>>();
    dbg!(&select_fields);

    let select_fields = vec![
        DField {
            path: Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice()),
            alias: Some("projectName".to_owned()),
        },
        DField {
            path: Dpath::from(vec!["hr", "employeesNest", "name"].as_slice()),
            alias: Some("employeeName".to_owned()),
        },
    ];

    let value = data.select_by_fields(&select_fields).unwrap();
    let list = to_list(value);
    dbg!(&list);

    let bindings_for_select = Bingings::from(select_fields.as_slice());

    let ss = list
        .iter()
        .filter_map(|value| match &sql.where_clause {
            Some(cond) if cond.eval(&value.to_owned(), &bindings, &bindings_for_select) => {
                Some(value.to_owned())
            }
            _ => None,
        })
        .collect::<Vec<JsonValue>>();
    dbg!(ss);

    dbg!("END OF FILE");
    Ok(())
}
