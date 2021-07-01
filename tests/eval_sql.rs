use std::str::FromStr;

use partiql::planner::evaluate;
use partiql::planner::Sql;

use partiql::value::PqlValue;

fn get_sql_data_output(qi: &str) -> anyhow::Result<(Sql, PqlValue, PqlValue)> {
    let sql = {
        let input = std::fs::read_to_string(format!("samples/{}.sql", qi)).unwrap();
        Sql::from_str(&input)?
    };

    let data = {
        let input = std::fs::read_to_string(format!("samples/{}.env", qi)).unwrap();
        PqlValue::from_str(&input)?
    };

    let output = {
        let input = std::fs::read_to_string(format!("samples/{}.output", qi)).unwrap();
        let v = input.split("---").collect::<Vec<_>>();
        let input = v.first().unwrap();
        PqlValue::from_str(&input)?
    };

    Ok((sql, data, output))
}

#[test]
fn q1() -> anyhow::Result<()> {
    let (sql, data, output) = get_sql_data_output("q1")?;
    let res = evaluate(sql, data);
    assert_eq!(res, output);
    Ok(())
}

#[test]
fn q2() -> anyhow::Result<()> {
    let (sql, data, output) = get_sql_data_output("q2")?;
    let res = evaluate(sql, data);
    assert_eq!(res, output);
    Ok(())
}

#[test]
fn q3() -> anyhow::Result<()> {
    let (sql, data, output) = get_sql_data_output("q3")?;
    let res = evaluate(sql, data);
    assert_eq!(res, output);
    Ok(())
}

// #[test]
// fn q4() -> anyhow::Result<()> {
//     let (sql, data, output) = get_sql_data_output("q4")?;
//     let res = evaluate(&sql, &data);
//     assert_eq!(res, output);
//     Ok(())
// }

#[test]
fn q5() -> anyhow::Result<()> {
    let (sql, data, output) = get_sql_data_output("q5")?;
    let res = evaluate(sql, data);
    assert_eq!(res, output);
    Ok(())
}

// #[test]
// fn q6() -> anyhow::Result<()> {
//     let (sql, data, output) = get_sql_data_output("q6")?;
//     let res = evaluate(&sql, &data);
//     assert_eq!(res, output);
//     Ok(())
// }

// #[test]
// fn q7() -> anyhow::Result<()> {
//     let (sql, data, output) = get_sql_data_output("q7")?;
//     let res = evaluate(&sql, &data);
//     assert_eq!(res, output);
//     Ok(())
// }
