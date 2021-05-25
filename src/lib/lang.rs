use std::str::FromStr;

use parse_display::{Display, FromStr};

use crate::models::JsonValue;

#[derive(Display, FromStr, PartialEq, Clone, Debug)]
#[display(style = "snake_case")]
pub enum LangType {
    Json,
    Toml,
    Xml,
    Yaml,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lang {
    pub data: JsonValue,
    pub lang_type: LangType,
    pub origin: String,
}

impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        if let Ok(data) = serde_json::from_str::<JsonValue>(&input) {
            Ok(Self {
                data,
                lang_type: LangType::Json,
                origin: input.to_string(),
            })
        } else if let Ok(data) = toml::from_str::<JsonValue>(&input) {
            Ok(Self {
                data,
                lang_type: LangType::Toml,
                origin: input.to_string(),
            })
        } else if let Ok(data) = quick_xml::de::from_str::<JsonValue>(&input) {
            Ok(Self {
                data,
                lang_type: LangType::Xml,
                origin: input.to_string(),
            })
        } else if let Ok(data) = serde_yaml::from_str::<JsonValue>(&input) {
            Ok(Self {
                data,
                lang_type: LangType::Yaml,
                origin: input.to_string(),
            })
        } else {
            anyhow::bail!("not supported")
        }
    }
}

impl Lang {
    pub fn print(&self) {
        let s = match self.lang_type {
            LangType::Json => serde_json::to_string_pretty(&self.data).unwrap(),
            _ => self.origin.to_owned(),
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
