use regex::Regex;

use partiql::models::JsonValue;
use partiql::pqlon_parser as parser;

fn main() {
    let r = parse();
    dbg!(r);
}

fn parse() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("samples/q1.env").unwrap();
    let model = parser::pql_model(&input)?;
    dbg!(&model);
    let s = serde_partiql::to_string(&model).unwrap();
    println!("{}", s);
    Ok(())
}
