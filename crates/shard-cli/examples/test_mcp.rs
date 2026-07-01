use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let mut child = Command::new("cargo")
        .args(["run", "-p", "shard-cli"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("Không thể chạy shard-cli");

    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();

    let requests = vec![
        serde_json::json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}),
        serde_json::json!({"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}),
        serde_json::json!({"jsonrpc":"2.0","id":3,"method":"tools/call","params":{"name":"shard_explorer","arguments":{"query":"AI","limit":3}}}),
        serde_json::json!({"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"shard_explorer","arguments":{"query":"knowledge"}}}),
    ];

    let input: String = requests
        .iter()
        .map(|r| serde_json::to_string(r).unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    stdin.write_all(input.as_bytes()).unwrap();
    drop(stdin);

    let output = std::io::read_to_string(stdout).unwrap();
    let responses: Vec<&str> = output.lines().collect();

    for (i, resp) in responses.iter().enumerate() {
        let parsed: serde_json::Value = serde_json::from_str(resp).unwrap();
        println!("--- Response {} ---", i + 1);
        println!("{}", serde_json::to_string_pretty(&parsed).unwrap());
        println!();
    }

    child.wait().unwrap();
}
