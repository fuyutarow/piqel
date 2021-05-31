use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::str::FromStr;

use indexmap::IndexMap as Map;
use ordered_float::OrderedFloat;
use parse_display::{Display, FromStr};
use polars::datatypes::AnyValue;
use polars::prelude::CsvReader;
use polars::prelude::*;
use rayon::prelude::*;

use crate::value::{BPqlValue, PqlValue, TomlValue};

#[derive(Display, FromStr, PartialEq, Clone, Debug)]
#[display(style = "snake_case")]
pub enum LangType {
    Csv,
    Json,
    Toml,
    Yaml,
    Xml,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lang {
    pub data: PqlValue,
    pub text: String,
    pub from: LangType,
    pub to: LangType,
}

impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        if let Ok(data) = csvstr_to_pqlv(&input) {
            Ok(Self {
                data,
                text: input.to_string(),
                from: LangType::Csv,
                to: LangType::Csv,
            })
        } else if let Ok(data) = serde_json::from_str::<serde_json::value::Value>(&input) {
            // Json does not distinguish between Float and Int. For this reason, it it parsed once with serde_json::value::Value, not crate::value::PqlValue.
            Ok(Self {
                data: crate::value::json_value::to_pqlvalue(data),
                text: input.to_string(),
                from: LangType::Json,
                to: LangType::Json,
            })
        } else if let Ok(data) = toml::from_str::<PqlValue>(&input) {
            Ok(Self {
                data,
                text: input.to_string(),
                from: LangType::Toml,
                to: LangType::Toml,
            })
        } else if let Ok(data) = quick_xml::de::from_str::<PqlValue>(&input) {
            Ok(Self {
                data,
                text: input.to_string(),
                from: LangType::Xml,
                to: LangType::Xml,
            })
        } else if let Ok(data) = serde_yaml::from_str::<PqlValue>(&input) {
            Ok(Self {
                data,
                text: input.to_string(),
                from: LangType::Yaml,
                to: LangType::Yaml,
            })
        } else {
            anyhow::bail!("not supported")
        }
    }
}

impl Lang {
    pub fn sort_keys(&mut self) {
        let json = serde_json::to_string(&self.data).unwrap();
        let bdata = serde_json::from_str::<BPqlValue>(&json).unwrap();
        let bjson = serde_json::to_string(&bdata).unwrap();
        let data = serde_json::from_str::<PqlValue>(&bjson).unwrap();
        self.data = data;
    }

    pub fn print(&self) {
        let output = match (&self.to, &self.from == &self.to) {
            (LangType::Csv, _) => todo!(),
            (LangType::Json, _) => serde_json::to_string_pretty(&self.data).unwrap(),
            (_, true) => self.text.to_owned(),
            (LangType::Toml, _) => {
                let v = TomlValue::from(self.data.to_owned());
                toml::to_string_pretty(&v).unwrap()
            }
            (LangType::Yaml, _) => serde_yaml::to_string(&self.data).unwrap(),
            (LangType::Xml, _) => quick_xml::se::to_string(&self.data).unwrap(),
        };

        if atty::is(atty::Stream::Stdout) {
            let bytes = output.as_bytes().to_vec();
            let lang_type = self.to.to_string();

            bat::PrettyPrinter::new()
                .language(&lang_type)
                .input(bat::Input::from_bytes(&bytes))
                .print()
                .unwrap();
        } else {
            println!("{}", &output);
        }
    }
}

fn csvstr_to_pqlv(input: &str) -> anyhow::Result<PqlValue> {
    let c = std::io::Cursor::new(input.to_owned());
    let df = CsvReader::new(c).infer_schema(Some(100)).finish()?;
    Ok(PqlValue::from(df))
}
