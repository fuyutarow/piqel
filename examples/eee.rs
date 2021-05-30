use nom::bitvec::vec;
use partiql::pqlir_parser;
use partiql::sql::restrict;
use partiql::sql::DPath;
use partiql::value::PqlValue;

fn main() -> anyhow::Result<()> {
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

    dbg!(&data);
    // let path = DPath::from("hr.employeesNest.title");
    // let path = DPath::from("hr.employeesNest.projects");
    let path = DPath::from("hr.employeesNest.projects.name");
    let res = restrict(Some(data), &path, Some("%security%"));
    dbg!(res);
    // assert_eq!(res, Some(PqlValue::Array(vec![PqlValue::Boolean(true)])));

    Ok(())
}
