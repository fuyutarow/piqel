use std::collections::HashSet;
use std::str::FromStr;

use indexmap::IndexMap as Map;
use itertools::Itertools;
use nom::combinator::{map, recognize};
use nom::error::{Error, ErrorKind, ParseError};
use nom::number::complete::recognize_float;
use nom::IResult;
use ordered_float::OrderedFloat;

use partiql::lang::Lang;
use partiql::parser;
use partiql::pqlir_parser;
use partiql::sql::to_list;
use partiql::sql::Bindings;
use partiql::sql::FieldBook;
use partiql::sql::Proj;
use partiql::sql::Sql;
use partiql::sql::{evaluate, Expr};
use partiql::sql::{DPath, Field};
use partiql::value::PqlValue;

fn get_sql_data_output(qi: &str) -> anyhow::Result<(Sql, PqlValue, PqlValue)> {
    let sql = {
        let input = std::fs::read_to_string(format!("samples/{}.sql", qi)).unwrap();
        let sql = parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string(format!("samples/{}.env", qi)).unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let output = {
        let input = std::fs::read_to_string(format!("samples/{}.output", qi)).unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    Ok((sql, data, output))
}

fn main() -> anyhow::Result<()> {
    let (sql, data, output) = get_sql_data_output("q2")?;

    let fields = sql
        .from_clause
        .iter()
        .chain(sql.left_join_clause.iter())
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();

    let bindings = Bindings::from(fields.as_slice());

    let projs = sql
        .select_clause
        .into_iter()
        .map(|proj| Proj {
            expr: proj.expr.to_owned(),
            alias: Some(proj.target_field_name()),
        })
        .collect::<Vec<_>>();
    // dbg!(&projs);

    let source_field_name_list = projs
        .iter()
        .map(|proj| proj.source_field_name_set(&bindings))
        .fold(HashSet::default(), |acc, x| {
            acc.union(&x).map(String::from).collect::<HashSet<_>>()
        });

    let vv = source_field_name_list
        .iter()
        .filter_map(|s| {
            let path = DPath::from(s.as_str());
            data.select_by_path(&path)
        })
        .collect::<Vec<PqlValue>>();
    // dbg!(&vv);

    let selected_source = data
        .select_by_fields(
            source_field_name_list
                .into_iter()
                .inspect(|e| {
                    dbg!(e);
                })
                .map(|s| {
                    let mut field = parser::parse_field(&s).unwrap().1;
                    field.alias = Some(field.path.to_string());
                    field
                })
                .inspect(|e| {
                    dbg!(e);
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .unwrap();
    dbg!(&selected_source);
    let mut book = FieldBook::from(selected_source.to_owned());
    dbg!(&book);
    dbg!(&projs);

    book.project_fields(&projs, &bindings);
    dbg!(&book);

    let records = book.to_record();

    let list = records.into_pqlv();
    dbg!(&list);

    dbg!(&output);

    Ok(())
}
