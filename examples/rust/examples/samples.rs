use piqel;
use reqwest;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let body = reqwest::get("https://registry.npmjs.org/-/v1/search?text=query")
        .await?
        .json::<serde_json::Value>()
        .await?;
    let sql = r#"
SELECT
  objects.package.name, 
  objects.searchScore AS score 
ORDERED BY score
    "#;

    let data = piqel::engine::evaluate(sql, &serde_json::to_string(&body).unwrap(), "json", "json");
    dbg!(data);

    Ok(())
}
