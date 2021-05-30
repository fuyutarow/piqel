use crate::pqlir_parser;
use crate::sql::parser;
use crate::sql::re_from_str;
use crate::sql::Bindings;
use crate::sql::DPath;
use crate::value::PqlValue;
use collect_mac::collect;
use indexmap::IndexMap as Map;
use nom::FindSubstring;

pub fn restrict(value: Option<PqlValue>, path: &DPath, right: Option<&str>) -> Option<PqlValue> {
    match value {
        None => None,
        Some(PqlValue::Boolean(boolean)) if boolean => Some(PqlValue::Boolean(boolean)),
        Some(PqlValue::Boolean(_)) => None,
        Some(PqlValue::Null) => None,
        Some(PqlValue::Str(string)) => {
            if let Some(pattern) = right {
                if re_from_str(pattern).is_match(&string) {
                    dbg!("!1", right, &string);
                    Some(PqlValue::Str(string))
                } else {
                    dbg!("!2", right, &string);
                    None
                }
            } else {
                dbg!("!3");
                Some(PqlValue::Str(string))
            }
        }
        Some(PqlValue::Float(float)) => Some(PqlValue::Float(float)),
        Some(PqlValue::Array(array)) => {
            let arr = array
                .into_iter()
                .filter_map(|v| {
                    dbg!(&v);
                    let vv = restrict(Some(v), path, right);
                    dbg!(&vv);
                    vv

                    // if let Some((first, tail)) = &path.split_first() {
                    //     restrict(Some(v), tail, right)
                    // } else {
                    //     None
                    // }
                })
                .collect::<Vec<_>>();
            dbg!(&arr);

            if arr.len() > 0 {
                Some(PqlValue::Array(arr))
            } else {
                None
            }
        }
        Some(PqlValue::Object(mut object)) => {
            if let Some((first, tail)) = &path.split_first() {
                if let Some(value) = object.get(&first.to_string()) {
                    dbg!(&value);
                    match restrict(Some(value.to_owned()), &tail, right) {
                        Some(v) if tail.to_vec().len() > 0 => {
                            dbg!("#1-1", first, tail, &v);
                            // Some(value.to_owned())
                            let it = object.get_mut(&first.to_string()).unwrap();
                            *it = v.to_owned();
                            Some(PqlValue::Object(object))
                        }
                        Some(v) => {
                            dbg!("#1-2", first, tail);
                            Some(PqlValue::Object(object.to_owned()))
                        }
                        _ => {
                            dbg!("#2");
                            None
                        }
                    }
                } else {
                    dbg!("#3");
                    None
                }
            } else {
                dbg!("#4");
                Some(PqlValue::Object(object.to_owned()))
            }
        }
        _ => {
            dbg!(value);
            todo!();
        }
    }
}

#[cfg(test)]
mod tests {
    use collect_mac::collect;
    use indexmap::IndexMap as Map;

    use super::restrict;
    use crate::pqlir_parser;
    use crate::sql::DPath;
    use crate::value::PqlValue;

    #[test]
    fn boolean() -> anyhow::Result<()> {
        let data = pqlir_parser::pql_model(
            "
    <<true, false, null>>
   ",
        )?;

        let res = restrict(Some(data), &DPath::default(), None);
        assert_eq!(res, Some(PqlValue::Array(vec![PqlValue::Boolean(true)])));
        Ok(())
    }

    #[test]
    fn missing() -> anyhow::Result<()> {
        let data = pqlir_parser::pql_model(
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
        let res = restrict(Some(data), &DPath::from("top.b"), None);
        let expected = pqlir_parser::pql_model(
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
    fn pattern_string() -> anyhow::Result<()> {
        let data = pqlir_parser::pql_model(
            "
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
   ",
        )?;
        let path = DPath::from("hr.employeesNest.projects.name");
        let res = restrict(Some(data), &path, Some("%security%"));
        let expected = pqlir_parser::pql_model(
            "
{
    'hr': {
        'employeesNest': <<
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
        >>
    }
}
   ",
        )?;
        assert_eq!(res, Some(expected));

        Ok(())
    }
}
