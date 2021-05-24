use std::io::{self, Read};
use std::path::{Path, PathBuf};

use regex::Regex;
use structopt::StructOpt;

use partiql::{
    dsql_parser,
    models::JsonValue,
    pqlir_parser,
    sql::{run, to_list},
    sql::{Bindings, DField, DSql, Dpath},
};

fn main() -> anyhow::Result<()> {
    let data = serde_json::from_str::<JsonValue>(
        r#"
{
  "SHELL": "/bin/bash",
  "NAME": "my machine name",
  "PWD": "/home/fuyutarow/partiql-rs",
  "LOGNAME": "fuyutarow",
  "HOME": "/home/fuyutarow",
  "LANG": "C.UTF-8",
  "USER": "fuyutarow",
  "HOSTTYPE": "x86_64",
  "_": "/usr/bin/env"
}
"#,
    )?;

    let sql = dsql_parser::sql(
        "
SELECT NAME, SHELL
",
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
