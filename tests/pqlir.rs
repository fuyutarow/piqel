use partiql::pqlir_parser;

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
    match pqlir_parser::pql_model(&input) {
        Ok(_) => assert_eq!(true, true),
        Err(_) => assert_eq!(true, false),
    }
}
