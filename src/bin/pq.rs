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
    #[structopt(short, long)]
    file: PathBuf,

    /// sql [possible_values: "*.json"]
    #[structopt(short, long)]
    query: String,

    /// target config file
    #[structopt(short, long, possible_values(&["json", "partiql"]), default_value="partiql")]
    to: String,
}

fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
        Opt { file, query, to } => {
            let ext = file
                .extension()
                .unwrap_or(std::ffi::OsStr::new(""))
                .to_str();
            let data = match ext.as_deref() {
                Some("json") => {
                    let input = std::fs::read_to_string(file)?;
                    let data = serde_json::from_str::<JsonValue>(&input)?;
                    data
                }
                _ => todo!(),
            };

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
