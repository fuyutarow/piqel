use std::collections::VecDeque;
use std::str::FromStr;

use crate::parser;
use crate::sql::Env;
use crate::sql::Expr;
use crate::value::PqlValue;

#[derive(Debug, Clone, PartialEq)]
pub enum SelectorNode {
    String(String),
    Number(i64),
}

impl Default for SelectorNode {
    fn default() -> Self {
        Self::String(String::default())
    }
}

impl From<&str> for SelectorNode {
    fn from(s: &str) -> Self {
        Self::String(s.to_string())
    }
}

impl From<i64> for SelectorNode {
    fn from(i: i64) -> Self {
        Self::Number(i)
    }
}

impl From<SelectorNode> for String {
    fn from(node: SelectorNode) -> Self {
        match node {
            SelectorNode::String(s) => s,
            SelectorNode::Number(i) => format!("{}", i),
        }
    }
}

impl SelectorNode {
    pub fn to_string(&self) -> String {
        String::from(self.to_owned())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Selector {
    pub data: VecDeque<SelectorNode>,
}

impl FromStr for Selector {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> anyhow::Result<Self> {
        match parser::expressions::parse_selector(s) {
            Ok((_, r)) => Ok(r),
            Err(_err) => anyhow::bail!("failed"),
        }
    }
}

impl From<&[&str]> for Selector {
    fn from(ss: &[&str]) -> Self {
        let data = ss
            .iter()
            .map(|s| SelectorNode::String(s.to_string()))
            .collect::<VecDeque<_>>();
        Self { data }
    }
}

impl From<&[String]> for Selector {
    fn from(ss: &[String]) -> Self {
        let data = ss
            .iter()
            .map(|s| SelectorNode::String(s.to_string()))
            .collect::<VecDeque<_>>();
        Self { data }
    }
}

impl From<&str> for Selector {
    fn from(s: &str) -> Self {
        let data = s
            .to_string()
            .split(".")
            .map(|s| SelectorNode::String(s.to_string()))
            .collect::<VecDeque<_>>();
        Self { data }
    }
}

impl From<&[SelectorNode]> for Selector {
    fn from(nodes: &[SelectorNode]) -> Self {
        Self {
            data: nodes
                .into_iter()
                .map(|n| n.to_owned())
                .collect::<VecDeque<_>>(),
        }
    }
}

impl Selector {
    pub fn last(&self) -> Option<String> {
        if let Some(last) = self.to_vec().last() {
            Some(last.to_string())
        } else {
            None
        }
    }

    pub fn split_first(&self) -> Option<(SelectorNode, Self)> {
        let mut data = self.data.to_owned();

        if let Some(first) = data.pop_front() {
            Some((first, Self { data }))
        } else {
            None
        }
    }

    pub fn to_string(&self) -> String {
        self.data
            .clone()
            .into_iter()
            .map(|node| node.to_string())
            .collect::<Vec<String>>()
            .join(".")
    }

    pub fn to_vec(&self) -> Vec<SelectorNode> {
        self.data.clone().into_iter().collect::<Vec<SelectorNode>>()
    }

    pub fn expand_fullpath(&self, env: &Env) -> Self {
        if let Some((head, tail)) = self.split_first() {
            let mut selector = Selector::default();

            selector.data.append(
                &mut env
                    .expand_fullpath_as_selector(&Selector::from(vec![head].as_slice()))
                    .data,
            );
            selector.data.append(&mut tail.data.to_owned());
            selector
        } else {
            todo!()
        }
    }

    pub fn expand_fullpath2(&self, env: &Env) -> Self {
        env.expand_fullpath_as_selector(&self)
    }

    pub fn expand_abspath(&self, env: &Env) -> Self {
        if let Some((head, tail)) = self.split_first() {
            let mut selector = Selector::default();
            if head != SelectorNode::default() {
                selector.data.push_front(SelectorNode::default());
            }

            selector.data.append(
                &mut env
                    .expand_fullpath_as_selector(&Selector::from(vec![head].as_slice()))
                    .data,
            );
            selector.data.append(&mut tail.data.to_owned());
            selector
        } else {
            todo!()
        }
    }

    pub fn evaluate(&self, env: &Env) -> Option<PqlValue> {
        if let Some((head, tail)) = self.expand_fullpath(&env).split_first() {
            if let Some(expr) = env.get(head.to_string().as_str()) {
                dbg!(&self);
                match expr {
                    Expr::Value(value) => {
                        dbg!(&value, &tail);
                        let v = if tail.data.len() > 0 {
                            value.select_by_selector(&tail)
                        } else {
                            Some(value)
                        };
                        dbg!(&v);
                        v
                    }
                    Expr::Selector(selector) => {
                        let s = selector.expand_fullpath(&env);
                        s.evaluate(&env)
                    }
                    Expr::Star => todo!(),
                    Expr::Func(_) => todo!(),
                    Expr::Add(_, _) => todo!(),
                    Expr::Sub(_, _) => todo!(),
                    Expr::Mul(_, _) => todo!(),
                    Expr::Div(_, _) => todo!(),
                    Expr::Rem(_, _) => todo!(),
                    Expr::Exp(_, _) => todo!(),
                    Expr::Sql(_) => todo!(),
                }
            } else {
                self.expand_abspath(&env).evaluate(&env)
            }
        } else {
            unreachable!()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::planner::Drain;

    use crate::sql::Env;
    use crate::sql::Expr;
    use crate::sql::Field;
    use crate::sql::Selector;
    use crate::value::PqlValue;

    fn get_data() -> anyhow::Result<PqlValue> {
        PqlValue::from_str(
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
        )
    }

    #[test]
    fn test_eval_selector_fullpath() -> anyhow::Result<()> {
        let env = {
            let mut env = Env::default();
            let data = get_data()?;
            env.insert("", &Expr::Value(data));
            env
        };

        let selector = Selector::from_str(".hr.employeesNest.name")?;

        assert_eq!(
            selector.evaluate(&env),
            Some(PqlValue::from_str(
                r#"
[
  "Bob Smith",
  "Susan Smith",
  "Jane Smith"
]
"#
            )?)
        );
        Ok(())
    }

    #[test]
    fn test_eval_selector_aliaspath() -> anyhow::Result<()> {
        let env = {
            let mut env = Env::default();
            let data = get_data()?;
            env.insert("", &Expr::Value(data));
            let drain = Drain(vec![
                Field::from_str(r#"hr.employeesNest AS e"#)?,
                Field::from_str(r#"e.projects AS p"#)?,
            ]);
            drain.execute(&mut env);
            env
        };

        let selector = Selector::from_str("e.projects")?;
        assert_eq!(
            selector.evaluate(&env),
            Some(PqlValue::from_str(
                r#"
[
  [
    {
      "name": "AWS Redshift Spectrum querying"
    },
    {
      "name": "AWS Redshift security"
    },
    {
      "name": "AWS Aurora security"
    }
  ],
  [],
  [
    {
      "name": "AWS Redshift security"
    }
  ]
]
"#
            )?)
        );
        Ok(())
    }

    #[test]
    fn test_eval_selector_aliaspath2() -> anyhow::Result<()> {
        let env = {
            let mut env = Env::default();
            let data = get_data()?;
            env.insert("", &Expr::Value(data));
            let drain = Drain(vec![
                Field::from_str(r#"hr.employeesNest AS e"#)?,
                Field::from_str(r#"e.projects AS p"#)?,
            ]);
            drain.execute(&mut env);
            env
        };

        let selector = Selector::from_str("p")?;
        assert_eq!(
            selector.evaluate(&env),
            Some(PqlValue::from_str(
                r#"
    [
      [
        {
          "name": "AWS Redshift Spectrum querying"
        },
        {
          "name": "AWS Redshift security"
        },
        {
          "name": "AWS Aurora security"
        }
      ],
      [],
      [
        {
          "name": "AWS Redshift security"
        }
      ]
    ]
    "#
            )?)
        );
        Ok(())
    }

    #[test]
    fn test_eval_selector_num() -> anyhow::Result<()> {
        let env = {
            let mut env = Env::default();
            let data = get_data()?;
            env.insert("", &Expr::Value(data));
            let drain = Drain(vec![Field::from_str(r#"3 AS n"#)?]);
            drain.execute(&mut env);
            env
        };

        let selector = Selector::from_str("n")?;
        assert_eq!(selector.evaluate(&env), Some(PqlValue::from_str("3")?));
        Ok(())
    }
}
