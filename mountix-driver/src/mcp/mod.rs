pub mod server;

use crate::mcp::server::MountixMcpServer;
use crate::module::Modules;
use rmcp::transport::streamable_http_server::{
    session::local::LocalSessionManager, StreamableHttpServerConfig, StreamableHttpService,
};
use std::env;
use std::sync::Arc;

pub fn mcp_service(
    modules: Arc<Modules>,
) -> StreamableHttpService<MountixMcpServer, LocalSessionManager> {
    let modules_for_factory = modules.clone();
    let config = mcp_config();

    StreamableHttpService::new(
        move || Ok(MountixMcpServer::new(modules_for_factory.clone())),
        LocalSessionManager::default().into(),
        config,
    )
}

fn mcp_config() -> StreamableHttpServerConfig {
    let mut config = StreamableHttpServerConfig::default()
        .with_stateful_mode(parse_bool_env("MCP_STATEFUL_MODE", false))
        .with_json_response(parse_bool_env("MCP_JSON_RESPONSE", true));

    if let Ok(hosts) = env::var("MCP_ALLOWED_HOSTS") {
        let allowed_hosts: Vec<String> = hosts
            .split(',')
            .map(str::trim)
            .filter(|host| !host.is_empty())
            .map(str::to_string)
            .collect();
        if !allowed_hosts.is_empty() {
            config = config.with_allowed_hosts(allowed_hosts);
        }
    } else if let Ok(host) = env::var("HOST") {
        config = config.with_allowed_hosts(["localhost", "127.0.0.1", "::1", host.as_str()]);
    }

    config
}

fn parse_bool_env(key: &str, default: bool) -> bool {
    env::var(key)
        .ok()
        .and_then(|value| match value.to_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bool_env() {
        std::env::set_var("TEST_BOOL_TRUE", "true");
        std::env::set_var("TEST_BOOL_FALSE", "false");

        assert!(parse_bool_env("TEST_BOOL_TRUE", false));
        assert!(!parse_bool_env("TEST_BOOL_FALSE", true));
        assert!(parse_bool_env("UNDEFINED_BOOL", true));

        std::env::remove_var("TEST_BOOL_TRUE");
        std::env::remove_var("TEST_BOOL_FALSE");
    }
}
