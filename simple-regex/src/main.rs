use anyhow::{Context as _, Result};
use lambda_runtime::{handler_fn, Context};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{json, Value};

static PAT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let func = handler_fn(reg);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn reg(e: Value, _: Context) -> Result<Value> {
    let date = e
        .get("date")
        .context("event does not have date")?
        .as_str()
        .context("date is not string")?;
    let result = PAT.is_match(&date);
    Ok(json!({ "date": date, "result": result }))
}
