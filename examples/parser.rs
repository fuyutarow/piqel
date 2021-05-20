use partiql::parser;

fn main() {
    parse();
}

fn parse() -> anyhow::Result<()> {
    let input = "SELECT * FROM [1,2,3]";

    let _ = parser::sql(input)?;

    let input = "[1,2,3]";
    let r = parser::array(input)?;
    dbg!(r);

    Ok(())
}
