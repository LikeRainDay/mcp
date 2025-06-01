use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;
use crate::utils::nacos_config::{
    get_nacos_config_by, get_nacos_service_info_by,
};
use serde_json;

#[tool(
    name = "GetNacosConfigByClient",
    description = "Get Nacos configuration information for specified namespace and data_id",
    params(namespace = "Nacos namespace", data_id = "Nacos data id", group = "Nacos group, defaults to DEFAULT_GROUP")
)]
pub async fn get_nacos_config_by_client(namespace: String, data_id: String, group: Option<String>) -> Result<ToolResponseContent> {
    let group = group.unwrap_or_else(|| "DEFAULT_GROUP".to_string());
    let config = get_nacos_config_by(&namespace, &data_id, &group).await?;
    Ok(tool_text_content!(config))
}

#[tool(
    name = "GetNacosServiceInfoByClient",
    description = "Get Nacos registered service instance information for specified namespace and service name",
    params(namespace = "Nacos namespace", service_name = "Nacos service name", group = "Nacos group, leave empty to get all groups")
)]
pub async fn get_nacos_service_info_by_client(namespace: String, service_name: String, group: Option<String>) -> Result<ToolResponseContent> {
    let info = get_nacos_service_info_by(&namespace, &service_name, group).await?;
    Ok(tool_text_content!(serde_json::to_string_pretty(&info)?))
}
