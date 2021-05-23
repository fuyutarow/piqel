use std::collections::HashMap;

use itertools::Itertools;

use partiql::dsql_parser;
use partiql::models::JsonValue;
use partiql::pqlir_parser;
use partiql::sql::run;
use partiql::sql::to_list;
use partiql::sql::Bingings;
use partiql::sql::DField;
use partiql::sql::DSql as Sql;
use partiql::sql::DWhereCond;
use partiql::sql::Dpath;
use partiql::sql_parser;

#[test]
fn q1() -> anyhow::Result<()> {
    let sql = {
        let input = std::fs::read_to_string("samples/q1.sql").unwrap();
        let sql = dsql_parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let output = {
        let input = std::fs::read_to_string("samples/q1.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let res = run(sql, data);
    assert_eq!(res, output);
    Ok(())
}

#[test]
fn q2() -> anyhow::Result<()> {
    let sql = {
        let input = std::fs::read_to_string("samples/q2.sql").unwrap();
        let sql = dsql_parser::sql(&input)?;
        sql
    };

    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let output = {
        let input = std::fs::read_to_string("samples/q2.output").unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        let model = pqlir_parser::pql_model(&input)?;
        model
    };

    let res = run(sql, data);
    assert_eq!(res, output);
    Ok(())
}
