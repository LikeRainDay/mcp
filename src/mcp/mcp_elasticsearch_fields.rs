use crate::utils::nacos_config::get_es_fields_config as get_config;
use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;

#[tool(
    name = "GetEsFieldsConfig",
    description = "Retrieve the full configuration of all Elasticsearch indices as defined in the system, including index names, field mappings, and metadata. This is useful for automated tools (such as cursor) to discover available indices and their field structures for further queries or validation."
)]
pub async fn get_es_fields_config() -> Result<ToolResponseContent> {
    let config = get_config()?;
    let yaml = serde_yaml::to_string(&config)?;
    Ok(tool_text_content!(yaml))
}

#[tool(
    name = "GetEsIndexFields",
    description = "Given an Elasticsearch index name, return the detailed field configuration (mappings, types, and metadata) for that specific index as defined in the system configuration. This enables automated tools (such as cursor) to programmatically inspect index schemas and validate field usage.",
    params(index_name = "The name of the Elasticsearch index to query for field configuration.")
)]
pub async fn get_es_index_fields(index_name: String) -> Result<ToolResponseContent> {
    let config = get_config()?;
    let index = config
        .indices
        .iter()
        .find(|idx| idx.name == index_name)
        .ok_or_else(|| anyhow::anyhow!("Index '{}' not found in configuration", index_name))?;

    let yaml = serde_yaml::to_string(&index)?;
    Ok(tool_text_content!(yaml))
}
