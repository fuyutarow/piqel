use std::convert::TryFrom;
use std::io::{self, Read};
use std::path::PathBuf;
use std::str::FromStr;

use structopt::StructOpt;

use piqel::lang::{Lang, LangType};
use piqel::parser;
use piqel::planner::evaluate;
use piqel::sql::Sql;

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
    #[structopt(short, long, possible_values(&["csv", "json", "toml", "yaml", "xml", "partiql"]))]
    from: Option<String>,

    /// target config file
    #[structopt(short, long, possible_values(&["csv", "json", "toml", "yaml", "xml", "partiql"]))]
    to: Option<String>,

    /// sort keys of objects on output. it on works when --to option is json, currently
    #[structopt(short = "S", long)]
    sort_keys: bool,

    /// compact instead of pretty-printed output, only when outputting in JSON
    #[structopt(short, long)]
    compact: bool,
}

fn main() -> anyhow::Result<()> {
    let Opt {
        file_or_stdin,
        query,
        from,
        to,
        sort_keys,
        compact,
    } = Opt::from_args();
    let _ = {
        let input = if let Some(file) = file_or_stdin {
            std::fs::read_to_string(file)?
        } else {
            read_from_stdin()?
        };

        let mut lang = if let Some(s_lang_type) = from {
            let lang_type = LangType::from_str(&s_lang_type)?;
            Lang::from_as(&input, lang_type)?
        } else {
            Lang::from_str(&input)?
        };

        if let Some(t) = to {
            match LangType::from_str(&t) {
                Ok(lang_type) => lang.to = lang_type,
                Err(err) => eprintln!("not support"),
            }
        }

        if let Some(q) = query {
            let sql = Sql::from_str(&q)?;
            let result = evaluate(sql, lang.data);
            lang.data = result;
        }

        if lang.to == LangType::Json && sort_keys {
            lang.sort_keys();
        }

        lang.print(compact);
    };

    Ok(())
}
