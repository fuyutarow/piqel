use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use partiql::lang::{Lang, LangType};
use partiql::parser;
use partiql::sql;

fn read_from_stdin() -> anyhow::Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read_to_string(&mut buf)?;

    Ok(buf)
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// source text: file or standard input
    #[structopt()]
    file_or_stdin: Option<PathBuf>,

    /// sql
    #[structopt(short, long)]
    query: Option<String>,

    /// target config file
    #[structopt(short, long, possible_values(&["csv", "json", "toml", "yaml", "xml"]))]
    to: Option<String>,

    /// sort keys of objects on output. it on works when --to option is json, currently
    #[structopt(short = "S", long)]
    sort_keys: bool,
}

fn main() -> anyhow::Result<()> {
    let _ = match Opt::from_args() {
        Opt {
            file_or_stdin,
            query,
            to,
            sort_keys,
        } => {
            let input = if let Some(file) = file_or_stdin {
                std::fs::read_to_string(file)?
            } else {
                read_from_stdin()?
            };

            let mut lang = Lang::from_str(&input)?;
            if let Some(t) = to {
                match LangType::from_str(&t) {
                    Ok(lang_type) => lang.to = lang_type,
                    Err(err) => eprintln!("not support"),
                }
            }

            if let Some(q) = query {
                let sql = parser::sql(&q)?;
                let result = sql::evaluate(&sql, &lang.data);
                lang.data = result?;
                lang.colnames = sql.get_colnames();
            }

            if lang.to == LangType::Json && sort_keys {
                lang.sort_keys();
            }

            lang.print();
        }
    };

    Ok(())
}
