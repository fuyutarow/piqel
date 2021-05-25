use std::str::FromStr;

use parse_display::{Display, FromStr};
use serde_derive::Serialize;

use partiql::models::JsonValue;

#[derive(Serialize)]
struct Person {
    name: String,
    height: f64,
    adult: bool,
    children: Vec<Person>,
}

#[derive(Display, FromStr, PartialEq, Debug)]
#[display(style = "snake_case")]
enum LangType {
    // PartiqlIr,
    Json,
    Toml,
    Xml,
    Yaml,
}

struct Lang {
    data: String,
    lang_type: LangType,
}

impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        let lang_type: anyhow::Result<LangType> =
            if let Ok(s) = serde_json::from_str::<JsonValue>(&input) {
                Ok(LangType::Json)
            } else if let Ok(s) = toml::from_str::<JsonValue>(&input) {
                Ok(LangType::Toml)
            } else if let Ok(s) = quick_xml::de::from_str::<JsonValue>(&input) {
                Ok(LangType::Xml)
            } else if let Ok(s) = serde_yaml::from_str::<JsonValue>(&input) {
                Ok(LangType::Yaml)
            } else {
                anyhow::bail!("not supported")
            };

        Ok(Self {
            data: input.to_string(),
            lang_type: lang_type?,
        })
    }
}

impl Lang {
    fn print(&self) {
        let s = match self.lang_type {
            LangType::Json => serde_json::to_string_pretty(
                &serde_json::from_str::<JsonValue>(&self.data).unwrap(),
            )
            .unwrap(),
            _ => self.data.to_owned(),
        };

        let bytes = s.as_bytes().to_vec();
        let lang_type = self.lang_type.to_string();

        bat::PrettyPrinter::new()
            .language(&lang_type)
            .line_numbers(true)
            .grid(true)
            .header(true)
            .input(bat::Input::from_bytes(&bytes).name(&lang_type))
            .print()
            .unwrap();
    }
}

fn main() {
    let person = Person {
        name: String::from("Anne Mustermann"),
        height: 1.76f64,
        adult: true,
        children: vec![Person {
            name: String::from("Max Mustermann"),
            height: 1.32f64,
            adult: false,
            children: vec![],
        }],
    };

    // let input = quick_xml::se::to_string(&person).unwrap();
    let input = serde_json::to_string(&person).unwrap();
    // let input = serde_yaml::to_string(&person).unwrap();

    match Lang::from_str(&input) {
        Ok(lang) => {
            lang.print();
        }
        Err(err) => {
            eprintln!("{}", err);
        }
    }
}
