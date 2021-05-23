use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::DField;
use partiql::sql::Dpath;
use partiql::sql::Sql;
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
    dbg!(&sql);

    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };
    dbg!(&data);

    let field_list = &sql.select_clause;

    let path_list = field_list
        .into_iter()
        .map(|field| sql.get_full_path(field))
        .collect::<Vec<_>>();
    dbg!(&path_list);

    // dbg!(data.by_path(&["hr",]));
    // dbg!(data.by_path(&["hr", "employeesNest"]));
    let path = Dpath::from(vec!["hr", "employeesNest", "projects", "name"].as_slice());
    dbg!(data.select_by_path(path));

    let path = Dpath::from(vec!["hr", "employeesNest", "name"].as_slice());
    dbg!(data.select_by_path(path));

    let path = Dpath::from(vec!["hr", "employeesNest", "name"].as_slice());
    let field = DField {
        source: "".to_owned(),
        path,
        alias: None,
    };
    dbg!(data.select_by_fields(&[field]));

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

    dbg!(data.select_map_by_fields(&[
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

    let parent_path = Dpath::from(vec!["hr", "employeesNest"].as_slice());
    let sub_data = data.select_by_path(parent_path).unwrap();
    dbg!(&sub_data);

    dbg!(sub_data.select_by_fields(&[
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["projects", "name"].as_slice()),
            alias: Some("projectName".to_owned()),
        },
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["name"].as_slice()),
            alias: Some("employeeName".to_owned()),
        }
    ]));

    dbg!(sub_data.select_map_by_fields(&[
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["projects", "name"].as_slice()),
            alias: Some("projectName".to_owned()),
        },
        DField {
            source: "".to_owned(),
            path: Dpath::from(vec!["name"].as_slice()),
            alias: Some("employeeName".to_owned()),
        }
    ]));

    dbg!("END OF FILE");
    Ok(())
}
