use std::io::{self, Read};
use std::path::{Path, PathBuf};

use regex::Regex;
use structopt::StructOpt;

use partiql::models::JsonValue;
use partiql::pqlir_parser as parser;
use partiql::sql::Sql;
use partiql::sql_parser;

fn read_from_stdin() -> anyhow::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

#[derive(StructOpt, Debug)]
enum Opt {
    #[structopt(name = "from")]
    From {
        /// target config file
        #[structopt()]
        input: Option<String>,

        /// target config file
        #[structopt(short, long, possible_values(&["json", "partiql"]), default_value="partiql")]
        to: String,
    },
    #[structopt(name = "sql")]
    Sql {
        /// target config file
        #[structopt(short, long)]
        file: PathBuf,

        /// sql [possible_values: "*.json"]
        #[structopt(short, long)]
        query: String,

        /// target config file
        #[structopt(short, long, possible_values(&["json", "partiql"]), default_value="partiql")]
        to: String,
    },
}

fn main() -> anyhow::Result<()> {
    match Opt::from_args() {
        Opt::From { input, to } => {
            let input = input.unwrap_or(read_from_stdin()?);

            match to.as_str() {
                "json" => {
                    let model = parser::pql_model(&input)?;
                    let s = serde_json::to_string(&model).unwrap();
                    println!("{}", s);
                }
                _ => {
                    let model = serde_json::from_str::<JsonValue>(&input)?;
                    let s = serde_partiql::to_string(&model).unwrap();
                    println!("{}", s);
                }
            }
        }
        Opt::Sql { file, query, to } => {
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

            if let Some(output) = run(sql, data) {
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
            } else {
                eprintln!("failed");
            }
        }
    };

    Ok(())
}

fn run(sql: Sql, data: JsonValue) -> Option<JsonValue> {
    let from_clause = sql.from_clause.first().unwrap();
    let full_path = format!("{}.{}", from_clause.source, from_clause.path);
    let from_path = full_path.split(".").collect::<Vec<_>>();

    let rows = data.get_path(&from_path).unwrap();

    let field_list = sql.select_clause;
    let data = rows.select_map(&field_list).unwrap();

    let cond = sql.where_clause.unwrap();
    let data = data.filter_map(cond).unwrap();
    Some(data)
}
