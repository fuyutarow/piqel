use std::collections::HashMap;

use itertools::Itertools;

use crate::dsql_parser;
use crate::models::JsonValue;
use crate::pqlir_parser;
use crate::sql::to_list;
use crate::sql::Bingings;
use crate::sql::DField;
use crate::sql::DSql as Sql;
use crate::sql::DWhereCond;
use crate::sql::Dpath;
use crate::sql_parser;

pub fn run(sql: Sql, data: JsonValue) -> JsonValue {
    let fields = sql
        .select_clause
        .iter()
        .chain(sql.from_clause.iter())
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();
    let bindings = Bingings::from(fields.as_slice());

    let select_fields = sql
        .select_clause
        .iter()
        .map(|field| field.to_owned().full(&bindings))
        .collect::<Vec<_>>();
    let bindings_for_select = Bingings::from(select_fields.as_slice());

    let value = data.select_by_fields(&select_fields).unwrap();
    let list = to_list(value);

    let filtered_list = list
        .iter()
        .filter_map(|value| match &sql.where_clause {
            Some(cond) if cond.eval(&value.to_owned(), &bindings, &bindings_for_select) => {
                Some(value.to_owned())
            }
            _ => None,
        })
        .collect::<Vec<JsonValue>>();

    JsonValue::Array(filtered_list)
}
