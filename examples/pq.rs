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
  let input = r#"
{
  "name": "partiql-pokemon",
  "version": "0.202105.0",
  "array": [
    1,
    3,
    "ko"
  ],
  "private": true,
  "scripts": {
    "dev": "next",
    "build": "next build",
    "start": "next start",
    "prod": "next build && next start",
    "lint": "eslint . --fix -c .eslintrc.js --ext js,jsx,ts,tsx --ignore-pattern='!.*'",
    "type-check": "tsc"
  },
  "license": "MIT"
}
"#;

  let r = serde_json::from_str::<JsonValue>(&input);
  dbg!(&r);

  let mut lang = Lang::from_str(&input)?;
  lang.to = LangType::Toml;
  // lang.to = LangType::Yaml;

  dbg!(&lang);

  lang.print();

  Ok(())
}
