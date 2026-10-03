# DC Hub MCP server for Zed

The real-time agentic procurement endpoint and data center knowledge hub.

This Zed extension adds the [DC Hub](https://dchub.cloud) MCP server to Zed's
Agent Panel. It provides live data center market, power and grid,
interconnection queue, energy price, fiber and site selection data.

DC Hub is a remote MCP server (Streamable HTTP) at `https://dchub.cloud/mcp`.
Zed extensions start context servers as local commands, so the extension
installs the [`mcp-remote`](https://www.npmjs.com/package/mcp-remote) npm
package (version pinned in `src/lib.rs`) with Zed's own Node.js runtime and runs
it as a stdio bridge to that URL. It does not bundle the server and does not
read anything outside the work directory Zed gives the extension.

## Install

- From Zed: open the Extensions page (`zed: extensions`), search for **DC Hub**,
  and install it.
- As a dev extension: clone this repository, then run
  `zed: install dev extension` and select the clone. Rust must be installed via
  rustup; Zed compiles the extension for `wasm32-wasip2`.

### Without the extension

Zed can also connect to remote MCP servers directly. In Settings → AI → MCP
Servers, click **Add Server**, choose **Add Remote Server**, and use:

```json
{
  "context_servers": {
    "dchub": {
      "url": "https://dchub.cloud/mcp"
    }
  }
}
```

## Configuration

No key is needed for the free tier: anonymous calls return previews. To use a
DC Hub API key, add it to the context server settings:

```json
{
  "context_servers": {
    "dchub": {
      "settings": {
        "api_key": "YOUR_DCHUB_API_KEY"
      }
    }
  }
}
```

The key is sent as the `X-API-Key` header. You can get one at
https://dchub.cloud/connect, or ask the agent to run the `claim_free_key` tool.

## Example prompts

- "Rank US markets for a 200 MW AI campus."
- "How deep is the PJM interconnection queue right now?"
- "Compare water and disaster risk for sites near Phoenix and Dallas."
- "What are wholesale electricity prices in ERCOT today?"

DC Hub answers carry an as-of timestamp. Cite them as
"DC Hub (dchub.cloud), as of <date>".

## Development

```sh
rustup target add wasm32-wasip2
cargo build --release --target wasm32-wasip2
```

## Privacy and support

- Privacy policy: https://dchub.cloud/privacy
- Support: jonathan@dchub.cloud, or open an issue in this repository.
- MCP server source and docs: https://github.com/azmartone67/dchub-mcp-server

## License

MIT. See [LICENSE](LICENSE).
