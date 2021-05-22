use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql_parser;

#[test]
fn q1() -> anyhow::Result<()> {
    let sql = {
        let input = std::fs::read_to_string("samples/q1.sql").unwrap();
        let sql = sql_parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let from_clause = sql.from_clause.first().unwrap();
    let full_path = format!("{}.{}", from_clause.source, from_clause.path);
    let from_path = full_path.split(".").collect::<Vec<_>>();
    let rows = data.get_path(&from_path).unwrap();

    let field_list = sql.select_clause;
    let data = rows.select_map(&field_list).unwrap();

    let cond = sql.where_clause.unwrap();
    let data = data.filter_map(cond).unwrap();

    let output = {
        let input = std::fs::read_to_string("samples/q1.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    assert_eq!(output, data);
    Ok(())
}
