use crate::sql::field::Field;
use crate::sql::Expr;
use crate::value::PqlValue;

#[derive(Debug, Clone, PartialEq)]
pub enum WhereCond {
    Eq { expr: Expr, right: PqlValue },
    Like { expr: Expr, right: String },
}

impl Default for WhereCond {
    fn default() -> Self {
        Self::Eq {
            expr: Expr::default(),
            right: PqlValue::default(),
        }
    }
}

impl WhereCond {
    pub fn get_expr(&self) -> Expr {
        match &self {
            Self::Eq { expr, right: _ } => expr.to_owned(),
            Self::Like { expr, right: _ } => expr.to_owned(),
        }
    }
}

pub fn re_from_str(pattern: &str) -> regex::Regex {
    let regex_pattern = match (pattern.starts_with("%"), pattern.ends_with("%")) {
        (true, true) => {
            format!("{}", pattern.trim_start_matches("%").trim_end_matches("%"))
        }
        (true, false) => format!("{}$", pattern.trim_start_matches("%")),
        (false, true) => format!("^{}", pattern.trim_end_matches("%")),
        (false, false) => format!("^{}$", pattern),
    };
    let re = regex::Regex::new(&regex_pattern).unwrap();
    re
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::planner::Drain;
    use crate::sql::Env;
    use crate::sql::Expr;
    use crate::sql::Field;
    use crate::sql::Selector;
    use crate::sql::Sql;
    use crate::value::PqlValue;
    use ordered_float::OrderedFloat;

    #[test]
    fn test_filter_env() -> anyhow::Result<()> {
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
        env.insert("", &Expr::from(data));

        let val = env.get_by_selector(&Selector::from(".hr.employeesNest"));

        let vvv = match val {
            Some(PqlValue::Array(arr)) => arr
                .into_iter()
                .filter(|value| -> bool {
                    match value.select_by_selector(&Selector::from("id")) {
                        None => todo!(),
                        Some(value) => value > PqlValue::from(3.),
                    }
                })
                .collect::<Vec<_>>(),
            _ => todo!(),
        };
        dbg!(&vvv);
        let res = PqlValue::Array(vvv);

        assert_eq!(
            PqlValue::from_str(
                r#"
[
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
]
        "#
            )?,
            res,
        );

        Ok(())
    }
}
