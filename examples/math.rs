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
use partiql::sql::Bindings;
use partiql::sql::FieldBook;
use partiql::sql::Proj;
use partiql::sql::{evaluate, Expr};
use partiql::sql::{DPath, Field};
use partiql::value::PqlValue;

fn main() -> anyhow::Result<()> {
    let lang = {
        let s = include_str!("samples/ip_addr.json");
        Lang::from_str(s)?
    };

    let input = "3*3";
    let (_, expr) = parser::parse_expr(input)?;
    let r = expr.eval();
    dbg!(r);

    let sql = parser::sql(
        r#"
    SELECT address, mtu * ifindex AS tri, 2* 5 AS one
    WHERE addr_info.family LIKE "inet%"
    "#,
    )?;
    dbg!(&sql);

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

    let v = projs
        .iter()
        .map(|proj| proj.source_field_name_set(&bindings))
        .fold(HashSet::default(), |acc, x| {
            acc.union(&x).map(String::from).collect::<HashSet<_>>()
        });
    // dbg!(&v);

    let selected_source = lang
        .data
        .select_by_fields(
            v.into_iter()
                .map(|s| parser::parse_field(&s).unwrap().1)
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .unwrap();
    dbg!(&selected_source);
    let mut book = FieldBook::from(selected_source.to_owned());
    dbg!(&book);
    // book.project_fields(&projs);

    //     let records = book.to_record();

    //     let list = records.into_pqlv();
    //     dbg!(&list);

    Ok(())
}
