// use partiql::models;
use partiql::pqlir_parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let _input = r#"
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
    "#;
    let input = r#"<<
  {
    'employeeName': 'Bob Smith',
    'projectName': 'AWS Redshift security'
  },
  {
    'employeeName': 'Bob Smith',
    'projectName': 'AWS Aurora security'
  },
  {
    'employeeName': 'Jane Smith',
    'projectName': 'AWS Redshift security'
  }
>>"#;
    // let model = pqlir_parser::pql_model(input)?;
    // dbg!(model);
    // let input = std::fs::read_to_string("samples/q1.env").unwrap();
    // println!("..{}", &input);
    let model = pqlir_parser::pql_model(&input)?;
    dbg!(model);

    let right = "%hello";

    let pattern = match (right.starts_with("%"), right.ends_with("%")) {
        (true, true) => format!("{}", right.trim_start_matches("%").trim_end_matches("%")),
        (true, false) => format!("{}$", right.trim_start_matches("%")),
        (false, true) => format!("^{}", right.trim_end_matches("%")),
        (false, false) => format!("^{}$", right),
    };
    let re = regex::Regex::new(&pattern).unwrap();

    let input = "hello world";
    let r = re.is_match(input);
    dbg!(r);

    let input = "ok, hello";
    let r = re.is_match(input);
    dbg!(r);
    // println!("{}", &v);
    Ok(())
}
