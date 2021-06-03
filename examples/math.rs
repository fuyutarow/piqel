use std::str::FromStr;

use indexmap::IndexMap as Map;
use nom::combinator::{map, recognize};
use nom::error::{Error, ErrorKind, ParseError};
use nom::number::complete::recognize_float;
use nom::IResult;
use ordered_float::OrderedFloat;

use partiql::lang::Lang;
use partiql::parser;
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
    SELECT address, mtu * 3 AS tri, 2* 5 AS one
    "#,
    )?;
    dbg!(&sql);
    // evaluate(&sql, &PqlValue::default());

    let projs = sql
        .select_clause
        .to_owned()
        .into_iter()
        .map(|proj| Proj {
            expr: proj.expr.to_owned(),
            alias: Some(proj.get_colname()),
        })
        .collect::<Vec<_>>();
    dbg!(&projs);

    let value_selected_by_fields = lang
        .data
        .select_by_fields(
            vec![
                parser::parse_field("address")?.1,
                parser::parse_field("mtu")?.1,
            ]
            .as_slice(),
        )
        .unwrap();

    let mut book = FieldBook::from(value_selected_by_fields);
    dbg!(&book);

    projs.into_iter().for_each(|proj| {
        let target_name = proj.alias.to_owned().unwrap();
        let target_fields = proj.eval(&book);
        dbg!(target_name, target_fields);
    });

    // let target_fields = projs
    //     .into_iter()
    //     .map(|proj| (proj.alias.unwrap(), proj.expr.eval()))
    //     .collect::<Map<String, PqlValue>>();
    // dbg!(&target_fields);

    // let records = {
    //     let mut records = Vec::<Map<String, Vec<PqlValue>>>::new();
    //     for i in 0..book.target_size {
    //         let mut record = Map::<String, Vec<PqlValue>>::new();
    //         for key in &book.target_keys {
    //             let v = book
    //                 .target_fields
    //                 .get(key.as_str())
    //                 .unwrap()
    //                 .get(i)
    //                 .unwrap();
    //             match v {
    //                 PqlValue::Array(array) => {
    //                     record.insert(key.to_string(), array.to_owned());
    //                 }
    //                 _ => {
    //                     record.insert(key.to_string(), vec![v.to_owned()]);
    //                 }
    //             }
    //         }
    //         records.push(record);
    //     }
    //     records
    // };
    // dbg!(&records);

    // let col = book
    //     .fields
    //     .get("mtu")
    //     .unwrap()
    //     .into_iter()
    //     .map(|e| e.to_owned() * PqlValue::Int(5))
    //     .collect::<Vec<_>>();

    // dbg!(&col);
    // book.fields.insert("mitsu".to_string(), col.to_owned());
    // dbg!(&book);

    // dbg!(&records);
    // let record = records[0].to_owned();
    // dbg!(&record);
    // let r = record["mtu"] * 3;

    Ok(())
}
