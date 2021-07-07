use partiql::pqlir_parser;

#[test]
fn bag() {
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
    match pqlir_parser::pql_value(&input) {
        Ok(_) => assert_eq!(true, true),
        Err(_) => assert_eq!(true, false),
    }
}

#[test]
fn t1() {
    let input = r#"
    {
    'hr': {
        'employees': <<
            -- a tuple is denoted by { ... } in the PartiQL data model
            {
                "id": 3, 'name': 'Bob Smith',   'title': null },
            <<
                  1 , 3, 4>>,
            { 'id': 4, 'name': 'Susan Smith', 'title': 'Dev Mgr' },
            { 'id': 6, 'name': 'Jane Smith',  'title': 'Software Eng 2'}
        >>
    }
}"#;
    match pqlir_parser::pql_value(&input) {
        Ok(_) => assert_eq!(true, true),
        Err(_) => assert_eq!(true, false),
    }
}
