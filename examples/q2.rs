use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::Sql;
use partiql::sql_parser;

fn main() {
    parse();
}

fn run(sql: Sql, data: JsonValue) -> Option<JsonValue> {
    let from_clause = sql.from_clause.first().unwrap();
    let full_path = format!("{}.{}", from_clause.source, from_clause.path);
    let from_path = full_path.split(".").collect::<Vec<_>>();

    dbg!(&from_path);

    let rows = data.get_path(&from_path).unwrap();
    dbg!(&rows);

    let field_list = sql.select_clause;
    dbg!(&field_list);
    let data = rows.select_map(&field_list).unwrap();
    dbg!(&data);

    let cond = sql.where_clause.unwrap();
    dbg!(&cond);
    let data = data.filter_map(cond).unwrap();
    dbg!(&data);
    Some(data)
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

    // let p = partiql::sql::Field {
    //     source: "e".to_owned(),
    //     path: "projects".to_owned(),
    //     alias: Some("p".to_owned()),
    // };

    // let path_s = sql.get_full_path(p);
    // let path = &path_s.iter().map(AsRef::as_ref).collect::<Vec<&str>>();

    // let full_path = data
    let field_list = &sql.select_clause;

    for field in field_list {
        let path_s = sql.get_full_path(field);
        let path = path_s.iter().map(AsRef::as_ref).collect::<Vec<&str>>();
        dbg!(&path);
        let d = data.by_path(&path);

        dbg!(d);
    }

    // dbg!(data.by_path(&["hr",]));
    // dbg!(data.by_path(&["hr", "employeesNest"]));
    dbg!(data.by_path(&["hr", "employeesNest", "projects", "name"]));

    let d = data.by_path(&["hr", "employeesNest"]).unwrap();

    dbg!(&d.select_map(&[
        partiql::sql::Field {
            source: "".to_owned(),
            path: "name".to_owned(),
            alias: Some("employeeName".to_owned()),
        },
        partiql::sql::Field {
            source: "".to_owned(),
            path: "projects.name".to_owned(),
            alias: Some("projectName".to_owned()),
        }
    ]));

    dbg!(&data.select_map(&[
        partiql::sql::Field {
            source: "".to_owned(),
            path: "hr.employeesNest.name".to_owned(),
            alias: Some("employeeName".to_owned()),
        },
        partiql::sql::Field {
            source: "".to_owned(),
            path: "hr.employeesNest.projects.name".to_owned(),
            alias: Some("projectName".to_owned()),
        }
    ]));

    // let val = data.by_path(path);
    // dbg!(val);
    // let data = run(sql, data).unwrap();
    // dbg!(&data);

    // let output = {
    //     let input = std::fs::read_to_string("samples/q2.output").unwrap();
    //     let v = input.split("---").collect::<Vec<_>>();
    //     let input = v.first().unwrap();
    //     let model = pqlir_parser::pql_model(&input)?;
    //     model
    // };
    // dbg!(&output);

    // assert_eq!(output, data);

    dbg!("--------");

    Ok(())
}
