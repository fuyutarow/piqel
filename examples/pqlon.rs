use regex::Regex;

use partiql::models::JsonValue;
use partiql::pqlon_parser as parser;

fn main() {
    let r = parse();
}

fn parse() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("samples/q1.env").unwrap();
    let model = parser::pql_model(&input)?;
    let s = serde_partiql::to_string(&model).unwrap();
    // println!("{}", s);
    let s = serde_json::to_string(&model).unwrap();
    // println!("{}", s);

    let m = serde_json::from_str::<JsonValue>(&s)?;
    dbg!(m);
    dbg!("----");
    Ok(())
}
