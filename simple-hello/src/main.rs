use anyhow::Result;
use lambda_runtime::{handler_fn, Context};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let func = handler_fn(hello);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn hello(_: Value, _: Context) -> Result<Value> {
    Ok(json!({"message": "Hello, World!"}))
}
