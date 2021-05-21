use regex::Regex;

use partiql::pqlon_parser as parser;

fn main() {
    let r = parse();
    dbg!(r);
}

fn parse() -> anyhow::Result<parser::JsonValue> {
    let input = std::fs::read_to_string("samples/q1.env").unwrap();
    parser::pql_model(&input)
}
