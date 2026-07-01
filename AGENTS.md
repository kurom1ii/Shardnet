# Shardnet — Code Rules

## General
- No comments (unless absolutely needed for complex logic)
- No unit tests (unless requested)
- Code names (struct, enum, fn) use English snake_case

## Project Structure
```
crates/
├── shard-types/     ← ShardId, ShardInfo, ShardDescription (shared)
└── shard-cli/       ← ShardContent + Shard struct (for self-hosted providers)
```

## Module Rules
- Each file has one clear responsibility
- `types.rs` — struct/enum definitions
- `mcp.rs` — dispatch JSON-RPC methods
- `tools.rs` — tool definitions + exec logic
- `routes.rs` — HTTP handlers (SSE, messages)
- `main.rs` — bootstrap + router setup

## Build
- `edition = "2024"`
- Use workspace dependencies centralized in root `Cargo.toml`

## API Conventions
- MCP protocol: JSON-RPC 2.0 over SSE transport
- Tool response format: `{ "content": [{ "type": "text", "text": "..." }] }`
