use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use partiql::dsql_parser as sql_parser;
use partiql::lang::{Lang, LangType};
use partiql::sql::run;
use partiql::value::JsonValue;

use collect_mac::collect;

use std::collections::HashMap as Map;

fn main() -> anyhow::Result<()> {
  let input = include_str!("samples/pokemons.json");
  let _input = r#"
[
  {
    "id": "001",
    "name": "Bulbasaur",
    "classification": "Seed Pokémon",
    "types": [
      "Grass",
      "Poison"
    ],
    "weight": {
      "minimum": "6.04kg",
      "maximum": "7.76kg"
    },
    "fleeRate": 0.1
  },
  {
    "id": "002",
    "name": "Ivysaur",
    "classification": "Seed Pokémon",
    "types": [
      "Grass",
      "Poison"
    ],
    "weight": {
      "minimum": "11.38kg",
      "maximum": "14.63kg"
    },
    "fleeRate": 0.07
  }
]
  "#;

  let r = serde_json::from_str::<JsonValue>(&input);
  dbg!(&r);

  let mut lang = Lang::from_str(&input)?;
  // lang.to = LangType::Toml;
  lang.to = LangType::Yaml;

  dbg!(&lang);

  lang.print();

  // let q = r#"
  // SELECT
  // "#;

  // let sql = sql_parser::sql(&q)?;
  // let result = run(&sql, &lang.data);
  // lang.data = result;

  Ok(())
}
