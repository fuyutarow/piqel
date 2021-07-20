use itertools::Itertools;

use crate::sql::Env;
use crate::sql::Expr;
use crate::value::PqlValue;

#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    Count(Expr),
    Upper(Expr),
}

impl Func {
    pub fn expand_fullpath(self, env: &Env) -> Self {
        match self {
            Func::Count(expr) => Self::Count(expr.expand_fullpath(env)),
            Func::Upper(expr) => Self::Upper(expr.expand_fullpath(env)),
        }
    }

    pub fn evaluate(self, env: &Env) -> PqlValue {
        match self {
            Func::Count(expr) => {
                let v = expr.eval(env);
                match v {
                    PqlValue::Array(array) => PqlValue::Array({
                        let ns = array
                            .into_iter()
                            .map(|elem| match elem {
                                PqlValue::Array(a) => PqlValue::from(a.len()),
                                _ => unreachable!(),
                            })
                            .collect_vec();
                        ns
                    }),
                    _ => unreachable!(),
                }
            }
            Func::Upper(expr) => {
                let v = expr.eval(env);
                v.to_uppercase()
            }
        }
    }

    pub fn is_aggregation(&self) -> bool {
        match self {
            Func::Count(_) => true,
            Func::Upper(_) => false,
        }
    }
}

impl PqlValue {
    fn to_uppercase(self) -> Self {
        match self {
            Self::Array(array) => Self::Array(
                array
                    .into_iter()
                    .map(|v| v.to_uppercase())
                    .collect::<Vec<_>>(),
            ),
            Self::Str(s) => Self::Str(s.to_uppercase()),
            _ => self,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::os::raw::c_longlong;
    use std::str::FromStr;

    use indexmap::IndexMap as Map;

    use crate::planner::LogicalPlan;
    use crate::sql::Env;
    use crate::sql::Expr;
    use crate::sql::Selector;
    use crate::sql::Sql;
    use crate::value::PqlValue;

    #[test]
    fn test_uppercase() -> anyhow::Result<()> {
        let mut env = Env::from(PqlValue::from_str(
            r#"
[
    { 'id': 3, 'name': 'Bob Smith' },
    { 'id': 4, 'name': 'Susan Smith', 'title': 'Dev Mgr' },
    { 'id': 6, 'name': 'Jane Smith', 'title': 'Software Eng 2'}
]
"#,
        )?);
        let sql = Sql::from_str(r#"SELECT id, Upper(name) AS upper"#)?;
        let logical_plan = LogicalPlan::from(sql);
        let r = logical_plan.execute(&mut env);
        r.print();

        assert_eq!(
            r,
            PqlValue::from_str(
                r#"
[
  {
    'id': 3.0,
    'upper': 'BOB SMITH'
  },
  {
    'id': 4.0,
    'upper': 'SUSAN SMITH'
  },
  {
    'id': 6.0,
    'upper': 'JANE SMITH'
  }
]
                "#,
            )?
        );
        Ok(())
    }
}
