use partiql::pqlon_parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let input = "[1,2,3]";
    let input = "  { \"a\"\t: 42,
  \"b\": [ \"x\", \"y\", 12 ] ,
  \"c\": { \"hello\" : \"world\"
  }
  } ";

    let r = pqlon_parser::root::<pqlon_parser::VerboseError<&str>>(input)?;
    dbg!(r);

    Ok(())
}
