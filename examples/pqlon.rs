use regex::Regex;

use partiql::pqlon_parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let input = r#"-- PartiQL
{
    'hr': {
        'employees': <<
            -- commnet out
            { 'id': 3, 'name': 'Bob Smith',   'title': null },
            { 'id': 4, 'name': 'Susan Smith', 'title': 'Dev Mgr' },
            { 'id': 6, 'name': 'Jane Smith',  'title': 'Software Eng 2'}
        >>
    }
}
    "#;
    let re = Regex::new(r"(^|\n)\s*--[\w\s]*\n").unwrap();
    let input = re.replace_all(&input, "");
    println!("{}", &input);

    match pqlon_parser::root::<pqlon_parser::VerboseError<&str>>(&input) {
        Ok(r) => {
            dbg!(r);
        }
        Err(err) => {
            dbg!(err);
        }
    }

    Ok(())
}
