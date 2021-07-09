use nom::dbg_dmp;

use crate::sql::re_from_str;
use crate::sql::Env;
use crate::sql::Expr;
use crate::sql::Selector;
use crate::sql::WhereCond;
use crate::value::PqlValue;

#[derive(Debug, Default, Clone)]
pub struct Filter(pub Option<Box<WhereCond>>);

impl Filter {
    pub fn execute(self, value: PqlValue, env: &Env) -> PqlValue {
        match self.0 {
            None => value,
            Some(box WhereCond::Eq { expr, right }) => match expr {
                Expr::Selector(selector) => {
                    let selector = selector.expand_fullpath2(&env);
                    let cond = WhereCond::Eq {
                        expr: Expr::default(),
                        right: right.to_owned(),
                    };
                    value
                        .restrict(&selector, &Some(cond))
                        .expect("restricted value")
                }
                _ => {
                    let pp = expr.expand_fullpath(env);
                    let v = expr.eval(env);
                    todo!()
                }
            },
            Some(box WhereCond::Like { expr, right }) => match expr {
                Expr::Selector(selector) => {
                    let selector = selector.expand_fullpath2(&env);
                    let cond = WhereCond::Like {
                        expr: Expr::default(),
                        right: right.to_owned(),
                    };
                    value
                        .restrict(&selector, &Some(cond))
                        .expect("restricted value")
                }
                _ => {
                    todo!();
                }
            },
            _ => {
                dbg!(&self);
                todo!()
            }
        }
    }

    pub fn expand_fullpath(self, env: &Env) -> Self {
        match self.0 {
            Some(box cond) => Self(Some(Box::new(cond.expand_fullpath(env)))),
            None => Self(None),
        }
    }
}

pub fn restrict(
    value: Option<PqlValue>,
    path: &Selector,
    cond: &Option<WhereCond>,
) -> Option<PqlValue> {
    match value {
        Some(PqlValue::Array(array)) => {
            let arr = array
                .into_iter()
                .filter_map(|v| {
                    let vv = restrict(Some(v), &path, cond);
                    vv
                })
                .collect::<Vec<_>>();

            if arr.is_empty() {
                None
            } else {
                Some(PqlValue::Array(arr))
            }
        }
        Some(PqlValue::Object(mut object)) => {
            if let Some((head, tail)) = &path.split_first() {
                if let Some(value) = object.get(&head.to_string()) {
                    match restrict(Some(value.to_owned()), &tail, cond) {
                        Some(v) => {
                            let it = object.get_mut(&head.to_string()).unwrap();
                            *it = v.to_owned();
                            Some(PqlValue::Object(object))
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            } else {
                unreachable!()
            }
        }
        None => None,
        Some(PqlValue::Boolean(boolean)) if boolean => Some(PqlValue::Boolean(boolean)),
        Some(PqlValue::Boolean(_)) => None,
        Some(PqlValue::Null) => None,
        Some(value) => match cond {
            Some(WhereCond::Eq { expr: _, right }) => {
                if value == right.to_owned() {
                    Some(value)
                } else {
                    None
                }
            }
            Some(WhereCond::Like { expr: _, right }) => {
                if let PqlValue::Str(string) = &value {
                    if re_from_str(&right).is_match(&string) {
                        Some(value)
                    } else {
                        None
                    }
                } else {
                    todo!()
                }
            }
            _ => unreachable!(),
        },
    }
}

impl PqlValue {
    fn restrict2(self, cond: &WhereCond) -> Option<Self> {
        match &self {
            PqlValue::Array(array) => {
                let arr = array
                    .into_iter()
                    .filter_map(|child| {
                        let vv = child.to_owned().restrict2(&cond);
                        vv
                    })
                    .collect::<Vec<_>>();
                let res = Some(PqlValue::from(arr));
                dbg!(&res);
                res
            }
            PqlValue::Object(object) => {
                let obj = match cond.to_path().map(|selector| selector.split_first()) {
                    Some(Some((head, tail))) => {
                        let data = self.to_owned();
                        let dd = cond.as_expr().eval(&Env::from(data.to_owned()));
                        dbg!(&dd);
                        if let Some(src) = object.get(head.to_string().as_str()) {
                            if let Some(dist) = src.to_owned().restrict2(cond) {
                                let mut restricted = object.to_owned();
                                restricted.insert(head.to_string(), dist);
                                let m = Some(PqlValue::from(restricted));
                                m
                            } else {
                                None
                            }
                        } else {
                            todo!()
                        }
                    }
                    Some(None) => {
                        todo!()
                    }
                    _ => None,
                };
                dbg!(&obj);
                obj
            }
            value => match cond.to_owned() {
                WhereCond::Eq { expr, right } => {
                    if expr.eval(&Env::from(value.to_owned())) == right {
                        Some(value.to_owned())
                    } else {
                        None
                    }
                }
                WhereCond::Like { expr: _, right } => {
                    if let PqlValue::Str(string) = &value {
                        if re_from_str(&right).is_match(&string) {
                            Some(value.to_owned())
                        } else {
                            None
                        }
                    } else {
                        todo!()
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::restrict;
    use crate::pqlir_parser;
    use crate::sql::Env;
    use crate::sql::Expr;
    use crate::sql::Field;
    use crate::sql::Selector;
    use crate::sql::WhereCond;
    use crate::value::PqlValue;

    #[test]
    fn boolean() -> anyhow::Result<()> {
        let value = PqlValue::from_str(
            "
    <<true, false, null>>
   ",
        )?;

        let res = value.restrict(&Selector::default(), &None);
        assert_eq!(res, Some(PqlValue::from_str(r#"<<true>>"#)?));
        Ok(())
    }

    #[test]
    fn missing() -> anyhow::Result<()> {
        let value = PqlValue::from_str(
            "
{
    'top': <<
        {'a': 1, 'b': true, 'c': 'alpha'},
        {'a': 2, 'b': null, 'c': 'beta'},
        {'a': 3, 'c': 'gamma'}
    >>
}
   ",
        )?;
        let res = value.restrict(&Selector::from("top.b"), &None);
        let expected = pqlir_parser::pql_value(
            "
{
    'top': <<
        {'a': 1, 'b': true, 'c': 'alpha'}
    >>
}
   ",
        )?;
        assert_eq!(res, Some(expected));

        Ok(())
    }

    #[test]
    fn test_filter_scalar() -> anyhow::Result<()> {
        let value = PqlValue::from_str(
            "
<<
    {
        'id': 3,
        'name': 'Bob Smith',
        'title': null,
        'projects': [
            { 'name': 'AWS Redshift Spectrum querying' },
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
   ",
        )?;
        let cond = WhereCond::Eq {
            expr: Expr::from(Selector::from("id")),
            right: PqlValue::from(6.),
        };
        let res = value.restrict2(&cond);
        dbg!(&res);
        let expected = pqlir_parser::pql_value(
            "
[
    {
        'id': 6,
        'name': 'Jane Smith',
        'title': 'Software Eng 2',
        'projects': [ { 'name': 'AWS Redshift security' } ]
    }
]
   ",
        )?;
        assert_eq!(res, Some(expected));
        Ok(())
    }

    #[test]
    fn test_filter_objects() -> anyhow::Result<()> {
        let value = PqlValue::from_str(
            "
<<
    {
        'id': 3,
        'name': 'Bob Smith',
        'title': null,
        'projects': [
            { 'name': 'AWS Redshift Spectrum querying' },
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
   ",
        )?;
        let selector = Selector::from("projects.name");
        let cond = WhereCond::Like {
            expr: Expr::default(),
            right: "%security%".to_owned(),
        };
        let res = value.restrict(&selector, &Some(cond));
        let expected = pqlir_parser::pql_value(
            "
[
    {
        'id': 3,
        'name': 'Bob Smith',
        'title': null,
        'projects': [
            { 'name': 'AWS Redshift security' },
            { 'name': 'AWS Aurora security' }
        ]
    },
    {
        'id': 6,
        'name': 'Jane Smith',
        'title': 'Software Eng 2',
        'projects': [ { 'name': 'AWS Redshift security' } ]
    }
]
   ",
        )?;
        assert_eq!(res, Some(expected));
        Ok(())
    }

    #[test]
    fn test_filter_like() -> anyhow::Result<()> {
        let value = PqlValue::from_str(
            "
<<
    {
        'id': 3,
        'name': 'Bob Smith',
        'title': null,
        'projects': [
            'AWS Redshift Spectrum querying',
            'AWS Redshift security',
            'AWS Aurora security'
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
        'projects': [ 'AWS Redshift security' ]
    }
>>
       ",
        )?;
        let cond = WhereCond::Like {
            expr: Expr::from(Selector::from("projects")),
            right: "%security%".to_owned(),
        };
        let res = value.restrict2(&cond);
        dbg!(&res);
        let expected = pqlir_parser::pql_value(
            "
[
    {
        'id': 3,
        'name': 'Bob Smith',
        'title': null,
        'projects': [
            'AWS Redshift security',
            'AWS Aurora security'
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
        'projects': [ 'AWS Redshift security' ]
    }
]
           ",
        )?;
        assert_eq!(res, Some(expected));
        Ok(())
    }

    #[test]
    fn test_filter_even() -> anyhow::Result<()> {
        let value = PqlValue::from_str(
            "
[
    { 'n': 0 },
    { 'n': 1 },
    { 'n': 2 },
    { 'n': 3 }
]
       ",
        )?;

        // let env = Env::from(value.to_owned());
        let cond = WhereCond::Eq {
            expr: Expr::from_str("n%2")?,
            right: PqlValue::from(0.),
        };

        let res = value.restrict2(&cond);
        let expected = pqlir_parser::pql_value(
            "
[
    { 'n': 0 },
    { 'n': 2 }
]
                   ",
        )?;
        assert_eq!(res, Some(expected));
        Ok(())
    }
}
