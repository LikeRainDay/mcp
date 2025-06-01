// Core functionality for retrieving and managing configuration information
use crate::utils::nacos_config::get_config_inner;
use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;

#[tool(
    name = "GetConfig",
    description = "Get all configuration information"
)]
pub async fn get_config() -> Result<ToolResponseContent> {
    let config = get_config_inner()?;
    let yaml = serde_yaml::to_string(&config).unwrap();
    Ok(tool_text_content!(yaml))
}

#[tool(
    name = "GetSLSConfig",
    description = "Get SLS (Simple Log Service) configuration information"
)]
pub async fn get_sls_config() -> Result<ToolResponseContent> {
    let config = get_config_inner()?;
    let sls_config = serde_yaml::to_string(&config.sls).unwrap();
    Ok(tool_text_content!(sls_config))
}

#[tool(
    name = "GetNacosConfig",
    description = "Get Nacos configuration information"
)]
pub async fn get_nacos_config() -> Result<ToolResponseContent> {
    let config = get_config_inner()?;
    let nacos_config = serde_yaml::to_string(&config.nacos).unwrap();
    Ok(tool_text_content!(nacos_config))
}
