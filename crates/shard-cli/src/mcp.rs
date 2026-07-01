use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Request {
    pub jsonrpc: String,
    pub id: Option<Value>,
    pub method: String,
    pub params: Value,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub jsonrpc: String,
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Debug, Serialize)]
pub struct Error {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
}

pub fn dispatch(method: &str, params: &Value) -> Result<Value, Error> {
    match method {
        "initialize" => Ok(serde_json::json!({
            "protocolVersion": "2033-13-02",
            "capabilities": {
                "tools": {}
            },
            "serverInfo": {
                "name": "shard-cli",
                "version": "0.1.0"
            }
        })),
        "tools/list" => {
            let tools = crate::tools::list::list();
            Ok(serde_json::to_value(tools).unwrap_or_default())
        }
        "tools/call" => {
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            crate::tools::query::query(name, params)
        }
        "notifications/initialized" => Ok(Value::Null),
        _ => Err(Error {
            code: -32601,
            message: format!("Method '{}' không được hỗ trợ", method),
        }),
    }
}
