# rust-mcp
MCP server to query weather information from [Openweathermap](https://openweathermap.org) written in Rust.

It requires an API Key from Openweathermap.

## Debugging
### MCP Inspector
The communication with the server can be tested with the interactive [MCP Inspector](https://modelcontextprotocol.io/docs/2026-07-28/tools/inspector).
To launch the inspector with the server execute npx:
```
npx @modelcontextprotocol/inspector \
  -e API_KEY=<YOUR_API_KEY \
  cargo run
```