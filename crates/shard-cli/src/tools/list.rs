use crate::mcp::Tool;

pub fn list() -> Vec<Tool> {
    vec![Tool {
        name: "shard_explorer".into(),
        description: "Explore shards hosted by this provider. Returns shard metadata (hash_id, name, size, description) so AI agents can connect directly to download value_content.".into(),
        input_schema: serde_json::json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "Search keyword in shard name and description"
                },
                "limit": {
                    "type": "integer",
                    "description": "Maximum number of results",
                    "default": 10
                }
            }
        }),
    }]
}
