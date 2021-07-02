use std::str::FromStr;

use nom::bitvec::vec;
use partiql::parser;
use partiql::planner::Drain;
use partiql::planner::LogicalPlan;
use partiql::planner::Sql;
use partiql::sql::Env;
use partiql::sql::Expr;
use partiql::sql::Field;
use partiql::sql::Selector;
use partiql::value::PqlValue;

fn main() -> anyhow::Result<()> {
    let data = PqlValue::from_str(
        r#"
{
  'hr': {
      'employeesNest': <<
         {
          'id': 3,
          'name': 'Bob Smith',
          'title': null,
          'projects': [ { 'name': 'AWS Redshift Spectrum querying' },
                        { 'name': 'AWS Redshift security' },
                        { 'name': 'AWS Aurora security' }
                      ]
          },
          {
              'id': 4,
              'name': 'Susan Smith',
              'title': 'Dev Mgr',
              'projects': []
          },
          {
              'id': 6,
              'name': 'Jane Smith',
              'title': 'Software Eng 2',
              'projects': [ { 'name': 'AWS Redshift security' } ]
          }
      >>
    }
}
    "#,
    )?;
    let mut env = Env::default();
    env.insert("", &Expr::Value(data.to_owned()));

    let drain = Drain(vec![
        Field::from_str(r#"hr.employeesNest AS e"#)?,
        Field::from_str(r#"e.projects AS p"#)?,
    ]);
    drain.excute(&mut env);
    dbg!(&env);
    let e = Selector::from_str("e")?;
    let v_e = data.select_by_selector(&e);
    dbg!(&v_e);

    // let selector = Selector::from_str(r#"hr.employees.id"#);

    // if let Some(v) = env.get("") {
    //     v.selec
    // }

    // let d = plan.excute(PqlValue::default(), &mut env);
    // dbg!(&env);
    // dbg!(&d);

    Ok(())
}
