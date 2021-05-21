use partiql::pqlon_parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let input = r#"
{
    'hr': {
        'employees': <<
            { 'id': 3, 'name': 'Bob Smith',   'title': null },
            { 'id': 4, 'name': 'Susan Smith', 'title': 'Dev Mgr' },
            { 'id': 6, 'name': 'Jane Smith',  'title': 'Software Eng 2'}
        >>
    }
}
    "#;
    let input = r#"
    {
        "hr": {
            "employees": <<
                { "id": 4, "name": "Susan Smith", "title": "Dev Mgr" },
                { "id": 6, "name": "Jane Smith",  "title": "Software Eng 2"}
            >>
        }
    }
    "#;

    match pqlon_parser::root::<pqlon_parser::VerboseError<&str>>(input) {
        Ok(r) => {
            dbg!(r);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    Ok(())
}
