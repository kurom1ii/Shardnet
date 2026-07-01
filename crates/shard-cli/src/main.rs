use std::io::BufRead;

mod mcp;
mod tools;

fn main() {
    let stdin = std::io::stdin();
    eprintln!("shard-cli MCP stdio server started");

    for line in stdin.lock().lines() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("Lỗi đọc stdin: {}", e);
                break;
            }
        };

        if line.trim().is_empty() {
            continue;
        }

        let request: mcp::Request = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let err = mcp::Response {
                    jsonrpc: "2.0".into(),
                    id: None,
                    result: None,
                    error: Some(mcp::Error {
                        code: -32700,
                        message: format!("Parse error: {}", e),
                    }),
                };
                println!("{}", serde_json::to_string(&err).unwrap());
                continue;
            }
        };

        if &request.method == "notifications/initialized" {
            continue;
        }

        let result = mcp::dispatch(&request.method, &request.params);

        let response = match result {
            Ok(value) => mcp::Response {
                jsonrpc: "2.0".into(),
                id: request.id,
                result: Some(value),
                error: None,
            },
            Err(err) => mcp::Response {
                jsonrpc: "2.0".into(),
                id: request.id,
                result: None,
                error: Some(err),
            },
        };

        println!("{}", serde_json::to_string(&response).unwrap());
    }

    eprintln!("shard-cli MCP stdio server stopped");
}
