//! DC Hub MCP server for Zed.
//!
//! DC Hub is a remote MCP server (Streamable HTTP) at https://dchub.cloud/mcp.
//! Zed extensions launch context servers as local commands, so this extension
//! installs the `mcp-remote` npm package through Zed's own Node.js runtime and
//! runs it as a stdio bridge to that URL. Nothing else is installed or read.

use std::env;

use serde::Deserialize;
use zed_extension_api::{
    self as zed, serde_json, settings::ContextServerSettings, Command, ContextServerConfiguration,
    ContextServerId, Project, Result,
};

const SERVER_URL: &str = "https://dchub.cloud/mcp";
const PACKAGE_NAME: &str = "mcp-remote";
/// Pinned so every install runs the same bridge. Bump deliberately.
const PACKAGE_VERSION: &str = "0.14.3";
const PACKAGE_ENTRY: &str = "node_modules/mcp-remote/dist/proxy.js";

#[derive(Debug, Default, Deserialize)]
struct DcHubSettings {
    /// Optional DC Hub API key. The free tier works without one.
    #[serde(default)]
    api_key: Option<String>,
}

struct DcHubExtension;

impl DcHubExtension {
    fn ensure_bridge_installed(&self) -> Result<String> {
        let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed.as_deref() != Some(PACKAGE_VERSION) {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }
        let entry = env::current_dir()
            .map_err(|e| format!("cannot resolve the extension work directory: {e}"))?
            .join(PACKAGE_ENTRY);
        Ok(entry.to_string_lossy().into_owned())
    }
}

impl zed::Extension for DcHubExtension {
    fn new() -> Self {
        DcHubExtension
    }

    fn context_server_command(
        &mut self,
        context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let entry = self.ensure_bridge_installed()?;

        let settings = ContextServerSettings::for_project(context_server_id.as_ref(), project)?;
        let dchub: DcHubSettings = match settings.settings {
            Some(value) => serde_json::from_value(value).map_err(|e| e.to_string())?,
            None => DcHubSettings::default(),
        };

        let mut args = vec![
            entry,
            SERVER_URL.to_string(),
            "--transport".to_string(),
            "http-only".to_string(),
        ];
        let mut env_vars = Vec::new();
        if let Some(key) = dchub.api_key.filter(|k| !k.trim().is_empty()) {
            // mcp-remote expands ${VAR} in header values, which keeps the key
            // out of the process arguments.
            args.push("--header".to_string());
            args.push("X-API-Key:${DCHUB_API_KEY}".to_string());
            env_vars.push(("DCHUB_API_KEY".to_string(), key));
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env: env_vars,
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        Ok(Some(ContextServerConfiguration {
            installation_instructions: include_str!("../configuration/installation_instructions.md")
                .to_string(),
            default_settings: include_str!("../configuration/default_settings.jsonc").to_string(),
            settings_schema: SETTINGS_SCHEMA.to_string(),
        }))
    }
}

const SETTINGS_SCHEMA: &str = r#"{
  "type": "object",
  "properties": {
    "api_key": {
      "type": "string",
      "description": "Optional DC Hub API key, sent as the X-API-Key header. Leave empty to use the free tier."
    }
  },
  "additionalProperties": false
}"#;

zed::register_extension!(DcHubExtension);
