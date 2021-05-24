// use partiql::

use nom::{
    branch::alt,
    bytes::complete::{escaped, tag, take_till, take_while, take_while_m_n},
    character::complete::{
        alphanumeric1, char, digit0, digit1, multispace0, multispace1, one_of, space1,
    },
    character::is_alphabetic,
    combinator::{cut, map, map_res, opt, value},
    error::{context, ContextError, ParseError},
    multi::{many0, many1, many_m_n, separated_list0, separated_list1},
    number::complete::{double, float, i64 as parse_i64},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

use partiql::{
    dsql_parser, pqlir_parser,
    sql::{Bindings, DField, DSql, Dpath},
};

fn main() -> anyhow::Result<()> {
    let sql = dsql_parser::sql(
        "
SELECT hr.employees.id AS id,
       hr.employees.name AS employeeName,
       hr.employees.title AS title
FROM hr
",
    )?;
    dbg!(&sql);

    let data = {
        let input = std::fs::read_to_string("samples/q1.env").unwrap();
        pqlir_parser::pql_model(&input)?
    };
    dbg!(&data);

    let fields = sql
        .select_clause
        .iter()
        .chain(sql.from_clause.iter())
        // .chain(sql.left_join_clause.iter())
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();
    let bindings = Bindings::from(fields.as_slice());
    dbg!(&bindings);
    dbg!(&fields);

    let field = DField {
        path: Dpath::from("hr.employees.id"),
        alias: None,
    };
    let r = field.full(&bindings);
    dbg!(r);

    let select_fields = sql
        .select_clause
        .iter()
        .inspect(|e| {
            dbg!("$1", e);
        })
        .map(|field| field.to_owned().full(&bindings))
        .inspect(|e| {
            dbg!("$2", e);
        })
        .map(|field| field.to_owned().full(&bindings))
        .collect::<Vec<_>>();
    let bindings_for_select = Bindings::from(select_fields.as_slice());
    dbg!(&bindings_for_select);
    dbg!(&select_fields);

    let values = data.select_by_fields(&select_fields);
    dbg!(&values);

    Ok(())
}
