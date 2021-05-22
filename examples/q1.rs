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
    dbg!(&data);

    // let s = data.get("hr");
    // dbg!(s);

    // let data = data.get("hr").unwrap();
    // dbg!(&data);
    // let data = data.get("employees");
    // dbg!(&data);

    let s = data.get_path(&["hr".to_string(), "employees".to_string()]);
    dbg!(s);

    // match data {
    //     JsonValue::Object(map) => {
    //         let s = map.get("hr");
    //         dbg!(s);
    //     }
    //     _ => {}
    // }

    let output = {
        let input = std::fs::read_to_string("samples/q1.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = parser::pql_model(&input)?;
        model
    };
    dbg!(&output);

    Ok(())
}
