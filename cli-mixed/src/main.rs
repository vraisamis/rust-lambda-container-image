use anyhow::{Context as _, Result};
use clap::{Parser, Subcommand};
use lambda_runtime::{handler_fn, Context};
use once_cell::sync::Lazy;
use regex::Regex;
use serde_json::{json, Value};

static PAT: Lazy<Regex> = Lazy::new(|| Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap());

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Hello,
    Reg,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args = Cli::parse();

    match &args.command {
        &Command::Hello => {
            let func = handler_fn(hello);
            lambda_runtime::run(func).await?;
        }
        &Command::Reg => {
            let func = handler_fn(reg);
            lambda_runtime::run(func).await?;
        }
    }

    Ok(())
}

async fn hello(_: Value, _: Context) -> Result<Value> {
    Ok(json!({"message": "Hello, World!"}))
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
