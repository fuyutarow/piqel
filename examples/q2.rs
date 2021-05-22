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

    let path_list = field_list
        .into_iter()
        .map(|field| sql.get_full_path(field))
        .collect::<Vec<_>>();
    dbg!(&path_list);

    // dbg!(data.by_path(&["hr",]));
    // dbg!(data.by_path(&["hr", "employeesNest"]));
    dbg!(data.by_path(&["hr", "employeesNest", "projects", "name"]));

    let (parent_path, child_path_list) = partiql::utils::split_parent_children(path_list);
    dbg!(&parent_path);
    dbg!(&child_path_list);

    let target_fields = child_path_list
        .into_iter()
        .zip(field_list.into_iter())
        .map(|(path, field)| partiql::sql::Field {
            source: "".to_owned(),
            // path: path.into_iter().collect::<String>(),
            path: path.join("."),
            alias: field.alias.to_owned(),
        })
        .collect::<Vec<_>>();

    let d = data.by_path(&["hr", "employeesNest"]).unwrap();

    // let r = d.select_map(target_fields.as_slice());
    let r = d.select_map(&target_fields);
    dbg!(r);

    let output = {
        let input = std::fs::read_to_string("samples/q2.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };
    dbg!(&output);

    // assert_eq!(output, data);

    dbg!("--------");

    Ok(())
}
