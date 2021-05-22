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

    let data = run(sql, data).unwrap();

    let output = {
        let input = std::fs::read_to_string("samples/q2.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        dbg!(&v);
        let input = v.first().unwrap();
        dbg!(&input);
        println!("{}", &input);
        let model = pqlir_parser::pql_model(&input)?;
        dbg!(&model);
        model
    };
    dbg!(&output);

    assert_eq!(output, data);

    dbg!("--------");

    Ok(())
}
