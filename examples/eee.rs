use nom::bitvec::vec;
use partiql::pqlir_parser;
use partiql::sql::restrict;
use partiql::sql::DPath;
use partiql::value::PqlValue;

fn main() -> anyhow::Result<()> {
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

    dbg!(&data);
    let res = restrict(Some(data), &DPath::from("top.b"), None);
    dbg!(res);
    // assert_eq!(res, Some(PqlValue::Array(vec![PqlValue::Boolean(true)])));
    let expected = pqlir_parser::pql_model(
        "
    <<
        {'a': 1, 'b': true, 'c': 'alpha'}
    >>
   ",
    )?;
    dbg!(expected);

    Ok(())
}
