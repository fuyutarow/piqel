use std::str::FromStr;

use parse_display::{Display, FromStr};

use crate::models::BJsonValue;
use crate::models::JsonValue;

#[derive(Display, FromStr, PartialEq, Clone, Debug)]
#[display(style = "snake_case")]
pub enum LangType {
    Json,
    Toml,
    Yaml,
    Xml,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Lang {
    pub data: JsonValue,
    pub text: String,
    pub from: LangType,
    pub to: LangType,
}

impl FromStr for Lang {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> anyhow::Result<Self> {
        if let Ok(data) = serde_json::from_str::<JsonValue>(&input) {
            Ok(Self {
                data,
                text: input.to_string(),
                from: LangType::Json,
                to: LangType::Json,
            })
        } else if let Ok(data) = toml::from_str::<JsonValue>(&input) {
            Ok(Self {
                data,
                text: input.to_string(),
                from: LangType::Toml,
                to: LangType::Toml,
            })
        } else if let Ok(data) = quick_xml::de::from_str::<JsonValue>(&input) {
            Ok(Self {
                data,
                text: input.to_string(),
                from: LangType::Xml,
                to: LangType::Xml,
            })
        } else if let Ok(data) = serde_yaml::from_str::<JsonValue>(&input) {
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
        let bdata = serde_json::from_str::<BJsonValue>(&json).unwrap();
        let bjson = serde_json::to_string(&bdata).unwrap();
        let data = serde_json::from_str::<JsonValue>(&bjson).unwrap();
        self.data = data;
    }

    pub fn print(&self) {
        let s = match self.to {
            LangType::Json => serde_json::to_string_pretty(&self.data).unwrap(),
            LangType::Toml => toml::to_string_pretty(&self.data).unwrap(),
            LangType::Yaml => serde_yaml::to_string(&self.data).unwrap(),
            LangType::Xml => quick_xml::se::to_string(&self.data).unwrap(),
        };

        let bytes = s.as_bytes().to_vec();
        let lang_type = self.to.to_string();

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
