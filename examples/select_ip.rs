use std::io::{self, Read};
use std::path::{Path, PathBuf};

use regex::Regex;
use structopt::StructOpt;

use partiql::{
  dsql_parser, pqlir_parser,
  sql::{run, to_list},
  sql::{Bindings, DField, DSql, Dpath},
  value::JsonValue,
};

fn main() -> anyhow::Result<()> {
  let data = serde_json::from_str::<JsonValue>(
    r#"
[
  {
    "addr_info": [
      {
        "family": "inet",
        "label": "lo",
        "local": "127.0.0.1",
        "valid_life_time": 4294967295
      },
      {
        "family": "inet6",
        "local": "::1",
        "valid_life_time": 4294967295
      }
    ],
    "address": "00:00:00:00:00:00",
    "broadcast": "00:00:00:00:00:00",
    "flags": [
      "LOOPBACK",
      "UP",
      "LOWER_UP"
    ],
    "group": "default",
    "txqlen": 1000
  },
  {
    "addr_info": [],
    "address": "0.0.0.0",
    "broadcast": "0.0.0.0",
    "flags": [
      "NOARP"
    ],
    "group": "default",
    "txqlen": 1000
  },
  {
    "addr_info": [
      {
        "broadcast": "172.22.255.255",
        "family": "inet",
        "label": "eth0",
        "local": "172.22.247.125",
        "valid_life_time": 4294967295
      },
      {
        "family": "inet6",
        "local": "fe80::215:5dff:fed8:2bc4",
        "valid_life_time": 4294967295
      }
    ],
    "address": "00:15:5d:d8:2b:c4",
    "broadcast": "ff:ff:ff:ff:ff:ff",
    "flags": [
      "BROADCAST",
      "MULTICAST",
      "UP",
      "LOWER_UP"
    ],
    "group": "default",
    "txqlen": 1000
  }
]
"#,
  )?;

  dbg!(&data);

  let sql = dsql_parser::sql(
    // SELECT address
    "
SELECT
  address,
  addr_info.family AS inet,
  addr_info.local
WHERE inet LIKE 'inet%'
",
    // FROM addr_info AS info
  )?;
  dbg!(&sql);

  let fields = sql
    .select_clause
    .iter()
    .chain(sql.from_clause.iter())
    .chain(sql.left_join_clause.iter())
    .map(|e| e.to_owned())
    .collect::<Vec<_>>();
  let bindings = Bindings::from(fields.as_slice());

  let field = DField {
    path: Dpath::from("hr.employees.id"),
    alias: None,
  };
  let r = field.full(&bindings);

  let select_fields = sql
    .select_clause
    .iter()
    .map(|field| field.to_owned().full(&bindings))
    .collect::<Vec<_>>();
  let bindings_for_select = Bindings::from(select_fields.as_slice());

  let value = data.select_by_fields(&select_fields).unwrap();
  dbg!(&value);

  let list = to_list(value);
  dbg!(&list);

  let filtered_list = list
    .iter()
    .filter_map(|value| match &sql.where_clause {
      Some(cond) if cond.eval(&value.to_owned(), &bindings, &bindings_for_select) => {
        Some(value.to_owned())
      }
      Some(_) => None,
      _ => Some(value.to_owned()),
    })
    .collect::<Vec<JsonValue>>();
  dbg!(&filtered_list);

  Ok(())
}
