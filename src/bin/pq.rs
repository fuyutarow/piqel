use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use regex::Regex;
use structopt::StructOpt;

use partiql::dsql_parser as sql_parser;
use partiql::lang::Lang;
use partiql::models::JsonValue;
use partiql::pqlir_parser as parser;
use partiql::sql::run;

fn read_from_stdin() -> anyhow::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// target config file
    #[structopt()]
    file_or_stdin: Option<PathBuf>,

    /// sql [possible_values: "*.json"]
    #[structopt(short, long)]
    query: Option<String>,

    // #[structopt(short, long, possible_values(&["json", "partiql"]), default_value="json")]
    // from: String,
    /// target config file
    #[structopt(short, long, possible_values(&["json", "partiql"]), default_value="json")]
    to: String,
}

fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
        Opt {
            file_or_stdin,
            query,
            to,
        } => {
            let input = if let Some(file) = file_or_stdin {
                std::fs::read_to_string(file)?
            } else {
                read_from_stdin()?
            };

            let mut lang = Lang::from_str(&input)?;

            if let Some(q) = query {
                let sql = sql_parser::sql(&q)?;
                let result = run(&sql, &lang.data);
                lang.data = result;
            }

            lang.print();
        }
    };

    Ok(())
}
