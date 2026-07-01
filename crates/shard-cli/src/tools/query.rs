use serde_json::Value;

use crate::mcp::Error;

pub fn query(name: &str, params: &Value) -> Result<Value, Error> {
    match name {
        "shard_explorer" => {
            let query = params
                .get("arguments")
                .and_then(|a| a.get("query"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let limit = params
                .get("arguments")
                .and_then(|a| a.get("limit"))
                .and_then(|v| v.as_u64())
                .unwrap_or(10);

            Ok(serde_json::json!({
                "content": [{
                    "type": "text",
                    "text": format!(
                        "Explore results for query='{}' (limit={}):\n\
                        No shards created yet.\n\
                        Use shard-cli to create a new shard: echo '{{\"name\":\"...\",\"data\":\"...\"}}' | shard-cli create",
                        query, limit
                    )
                }]
            }))
        }
        _ => Err(Error {
            code: -32601,
            message: format!("Tool '{}' not found", name),
        }),
    }
}
