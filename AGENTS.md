# Shardnet — Quy tắc code

## Nguyên tắc chung
- Không viết comments (trừ khi thực sự cần giải thích logic phức tạp)
- Không viết unit test (trừ khi được yêu cầu)
- Tiếng Việt có dấu cho naming, mô tả, tool description
- Code name (struct, enum, fn) dùng tiếng Anh snake_case

## Cấu trúc project
```
crates/
├── shard-server/    ← registry MCP server (chỉ metadata, không lưu content)
├── shard-types/     ← ShardId, ShardInfo, ShardDescription (dùng chung)
└── shard-cli/       ← ShardContent + Shard struct (dành cho provider tự host)
```

## Quy tắc module
- Mỗi file chỉ làm 1 nhiệm vụ rõ ràng
- `types.rs` — struct/enum định nghĩa
- `mcp.rs` — dispatch JSON-RPC methods
- `tools.rs` — tool definitions + exec logic
- `routes.rs` — HTTP handlers (SSE, messages)
- `main.rs` — bootstrap + router setup

## Build
- `edition = "2021"`
- Dùng workspace dependencies tập trung trong root `Cargo.toml`

## API conventions
- MCP protocol: JSON-RPC 2.0 qua SSE transport
- Tool response format: `{ "content": [{ "type": "text", "text": "..." }] }`
