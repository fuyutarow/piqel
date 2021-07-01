use indexmap::IndexMap as Map;
use itertools::Itertools;

use partiql::parser;
use partiql::pqlir_parser;
use partiql::sql::plan::Drain;
use partiql::sql::plan::Filter;
use partiql::sql::plan::Projection;
use partiql::sql::restrict;
use partiql::sql::Env;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::FieldBook;
use partiql::sql::Proj;
use partiql::sql::WhereCond;
use partiql::value::BPqlValue;
use partiql::value::PqlValue;

pub struct Evaluator {
    pub source: PqlValue,
    pub project: Vec<Proj>,
}

fn main() -> anyhow::Result<()> {
    let data = {
        let input = std::fs::read_to_string("samples/q2.env").unwrap();
        pqlir_parser::from_str(&input)?
    };
    let mut env = Env::default();
    env.insert_from_pqlval("", &data);
    Drain(parser::clauses::from(r#"FROM hr.employeesNest AS e"#)?.1).excute(&mut env);
    Drain(parser::clauses::left_join(r#"LEFT JOIN e.projects AS p"#)?.1).excute(&mut env);
    dbg!(&env);

    let condition = parser::clauses::parse_where(r#"WHERE p.name LIKE '%security%'"#)?.1;
    let data = Filter(Some(Box::new(condition))).execute(data, &env);

    let fields = parser::clauses::select2(
        r#"SELECT e.id AS id,
       e.name AS employeeName,
       e.title AS title,
       p.name AS projectName"#,
    )?
    .1
    .into_iter()
    .map(|field| field.expand_fullpath(&env))
    .collect::<Vec<Field>>();
    let projected = Projection(fields).execute(data);
    dbg!(projected);

    Ok(())
}
