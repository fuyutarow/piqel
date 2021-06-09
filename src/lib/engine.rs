use std::str::FromStr;

use crate::lang::{Lang, LangType};
use crate::parser;
use crate::sql;

pub fn evaluate(sql: &str, input: &str, from: &str, to: &str) -> anyhow::Result<String> {
    let from_lang_type = LangType::from_str(&from)?;
    let to_lang_type = LangType::from_str(&to)?;
    let mut lang = Lang::from_as(&input, from_lang_type)?;

    let sql = parser::sql(&sql)?;
    let result = sql::evaluate(&sql, &lang.data);
    lang.to = to_lang_type;
    lang.data = result;
    lang.colnames = sql.get_colnames();
    let output = lang.to_string(true)?;

    Ok(output)
}
