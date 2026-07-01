# Shardnet

**New kind of internet for AI Agents.**

Shardnet is a content-addressable protocol that lets AI agents discover, connect, and interact with executable knowledge units called **Shards** — text, code, media, memory, skills, or any digital entity.

## Architecture

```
┌──────────────┐     MCP/JSON-RPC     ┌──────────────┐
│   AI Agent   │ ◄──────────────────► │  shard-cli   │
└──────────────┘                      └──────┬───────┘
                                             │
                                    ┌────────▼───────┐
                                    │  shard-server  │  ← registry (metadata only)
                                    └────────────────┘
                                             │
                              ┌──────────────┼──────────────┐
                              ▼              ▼              ▼
                         ┌────────┐   ┌────────┐   ┌────────┐
                         │ Shard  │   │ Shard  │   │ Shard  │
                         └────────┘   └────────┘   └────────┘
```

| Component     | Role |
|---------------|------|
| `shard-cli`   | MCP stdio server — AI agents use it to search, connect, and execute Shards |
| `shard-server`| Registry — indexes Shard metadata (hash, tags, embedding, endpoint) |
| `shard-types` | Core types: `ShardId`, `ShardInfo`, `ShardDescription` |

## How It Works

1. **Register** — Content is hashed (SHA-512) and published to the registry with a vector embedding + summary
2. **Discover** — AI agents query the registry via semantic search or tags
3. **Connect** — Shards expose HTTP, WebSocket, TCP, or UDP endpoints
4. **Execute** — Agents call `shard_execute` to run code, load context, or interact with live entities

## Quick Start

```bash
# Build
cargo build --release

# Run CLI (MCP stdio server)
cargo run -p shard-cli

# Run registry server (separate repo)
git clone https://github.com/kurom1ii/shard-server.git
cd shard-server && cargo run
```

## Repository

| Repo | Visibility | Description |
|------|-----------|-------------|
| [Shardnet](https://github.com/kurom1ii/Shardnet) | Public | CLI, types, protocol |
| [shard-server](https://github.com/kurom1ii/shard-server) | Private | Registry MCP server |

## License

MIT
