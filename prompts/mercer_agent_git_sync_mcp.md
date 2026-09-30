# Git sync: intent and implementation

The earlier MCP-first proposal wanted one auditable route for file changes, commits and shared memory. That goal is useful, but the implementation in this repository does not deliver the proposed write-and-commit path.

## What was checked

During the September 30 source review:

- `mcp_gateway/main.go` selected a matching registry entry, then returned a placeholder instead of forwarding the request.
- `mcp_servers/filesystem_server.go` left write and delete operations blocked as unimplemented.
- Both memory-server implementations left memory writes blocked and did not commit changes to Git.

Those are source observations, not a live deployment receipt. Do not tell an agent to stage or commit through nonexistent MCP endpoints.

## The useful rule

Use the actual authorized editing and Git tools available in the current harness. Keep changes reviewable, preserve other contributors' work, record verification in the commit and respect the current publication approval. If a future gateway implements this flow, document and test the real route before making it the default.

A commit message needs a clear change and its evidence. It does not need an ASCII banner, mandatory poem or claim that an audit occurred simply because the tool has a styled response.

The [original proposal](https://github.com/RamenFast/SymbolOS_archive/blob/ead60385ef3170028fc91606de24efa45b392034/prompts/mercer_agent_git_sync_mcp.md) remains in the archive.

[Starting brief](README.md) · [Contribution guide](../CONTRIBUTING.md)
