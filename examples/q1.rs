use partiql::models::JsonValue;
use partiql::pqlon_parser as parser;
use partiql::sql_parser;
fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let sql = {
        let input = std::fs::read_to_string("samples/q1.sql").unwrap();
        let sql = sql_parser::sql(&input)?;
        sql
    };
    dbg!(&sql);

    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        let model = parser::pql_model(&input)?;
        model
    };
    // dbg!(&data);

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

    let output = {
        let input = std::fs::read_to_string("samples/q1.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = parser::pql_model(&input)?;
        model
    };
    dbg!(&output);

    assert_eq!(output, data);

    dbg!("--------");

    Ok(())
}
