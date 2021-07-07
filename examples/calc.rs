use partiql::parser;
use partiql::planner::LogicalPlan;
use partiql::sql::Env;
use partiql::sql::Sql;
use partiql::value::PqlValue;

fn main() -> anyhow::Result<()> {
    let se = parser::clauses::select(r#"SELECT 4 * a"#)?.1;
    dbg!(se);

    let mut sql = Sql::default();
    sql.select_clause = parser::clauses::select(r#"SELECT 4 * a AS aa"#)?.1;
    sql.from_clause = parser::clauses::from("FROM 3 as a")?.1;
    dbg!(&sql);
    let plan = LogicalPlan::from(sql);
    dbg!(&plan);

    let mut env = Env::default();
    let d = plan.execute(PqlValue::default(), &mut env);
    dbg!(&env);
    dbg!(&d);

    Ok(())
}
