use std::io::{self, Read};
use std::path::{Path, PathBuf};

use regex::Regex;
use structopt::StructOpt;

use partiql::dsql_parser as sql_parser;
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
    query: String,

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

            let data = serde_json::from_str::<JsonValue>(&input)?;

            let sql = sql_parser::sql(&query)?;

            let output = run(&sql, &data);

            let s = match to.as_str() {
                "json" => {
                    let s = serde_json::to_string(&output).unwrap();
                    s
                }
                _ => {
                    let s = serde_partiql::to_string(&output).unwrap();
                    s
                }
            };
            println!("{}", s);
        }
    };

    Ok(())
}
