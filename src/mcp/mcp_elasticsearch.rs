use anyhow::{anyhow, Result};
use elasticsearch::{
    auth::Credentials,
    http::transport::{SingleNodeConnectionPool, TransportBuilder},
    indices::{IndicesExistsParts, IndicesGetAliasParts, IndicesGetMappingParts},
    Elasticsearch as ES8,
};
use serde_json::Value;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;
use crate::utils::config::{AppConfig, ElasticsearchConfig};

pub enum MCPElasticsearch {
    V6(reqwest::Client, ElasticsearchConfig),
    V8(ES8, ElasticsearchConfig),
}

impl MCPElasticsearch {
    pub fn new(config: &ElasticsearchConfig) -> Result<Self> {
        match config.version.as_str() {
            "6.8" => {
                let mut client_builder = reqwest::Client::builder();

                if let (Some(username), Some(password)) = (&config.username, &config.password) {
                    let auth = base64::encode(format!("{}:{}", username, password));
                    client_builder = client_builder.default_headers({
                        let mut headers = reqwest::header::HeaderMap::new();
                        headers.insert(
                            reqwest::header::AUTHORIZATION,
                            format!("Basic {}", auth).parse().unwrap(),
                        );
                        headers
                    });
                }

                let client = client_builder.build()?;
                Ok(Self::V6(client, config.clone()))
            }
            "8.0" => {
                let url = url::Url::parse(&config.url)?;
                let conn_pool = SingleNodeConnectionPool::new(url);
                let mut transport_builder = TransportBuilder::new(conn_pool);

                if let (Some(username), Some(password)) = (&config.username, &config.password) {
                    transport_builder = transport_builder.auth(Credentials::Basic(
                        username.clone(),
                        password.clone(),
                    ));
                }

                let transport = transport_builder.build()?;
                let client = ES8::new(transport);
                Ok(Self::V8(client, config.clone()))
            }
            _ => Err(anyhow!("Unsupported Elasticsearch version: {}", config.version)),
        }
    }

    pub async fn index_exists(&self, index: &str) -> Result<bool> {
        match self {
            Self::V6(client, _) => {
                let response = client
                    .head(format!("{}/{}", self.get_base_url(), index))
                    .send()
                    .await?;
                Ok(response.status().is_success())
            }
            Self::V8(client, _) => {
                let response = client
                    .indices()
                    .exists(IndicesExistsParts::Index(&[index]))
                    .send()
                    .await?;
                Ok(response.status_code().is_success())
            }
        }
    }

    pub async fn get_index(&self, index: &str) -> Result<Value> {
        match self {
            Self::V6(client, _) => {
                let response = client
                    .get(format!("{}/{}", self.get_base_url(), index))
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
            Self::V8(client, _) => {
                let response = client
                    .indices()
                    .get(elasticsearch::indices::IndicesGetParts::Index(&[index]))
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
        }
    }

    pub async fn get_aliases(&self, index: &str) -> Result<Value> {
        match self {
            Self::V6(client, _) => {
                let response = client
                    .get(format!("{}/{}/_alias", self.get_base_url(), index))
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
            Self::V8(client, _) => {
                let response = client
                    .indices()
                    .get_alias(IndicesGetAliasParts::Index(&[index]))
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
        }
    }

    pub async fn get_mapping(&self, index: &str) -> Result<Value> {
        match self {
            Self::V6(client, _) => {
                let response = client
                    .get(format!("{}/{}/_mapping", self.get_base_url(), index))
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
            Self::V8(client, _) => {
                let response = client
                    .indices()
                    .get_mapping(IndicesGetMappingParts::Index(&[index]))
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
        }
    }

    pub async fn get_health(&self) -> Result<Value> {
        match self {
            Self::V6(client, _) => {
                let response = client
                    .get(format!("{}/_cluster/health", self.get_base_url()))
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
            Self::V8(client, _) => {
                let response = client
                    .cluster()
                    .health(elasticsearch::cluster::ClusterHealthParts::None)
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
        }
    }

    pub async fn search(&self, index: &str, query: &str) -> Result<Value> {
        match self {
            Self::V6(client, _) => {
                let query: Value = serde_json::from_str(query)?;
                let response = client
                    .post(format!("{}/{}/_search", self.get_base_url(), index))
                    .json(&query)
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
            Self::V8(client, _) => {
                let query: Value = serde_json::from_str(query)?;
                let response = client
                    .search(elasticsearch::SearchParts::Index(&[index]))
                    .body(query)
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                Ok(body)
            }
        }
    }

    pub async fn get_version(&self) -> Result<String> {
        match self {
            Self::V6(client, _) => {
                let response = client
                    .get(self.get_base_url())
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                let version = body["version"]["number"]
                    .as_str()
                    .unwrap_or("unknown");
                Ok(version.to_string())
            }
            Self::V8(client, _) => {
                let response = client
                    .info()
                    .send()
                    .await?;
                let body = response.json::<Value>().await?;
                let version = body["version"]["number"]
                    .as_str()
                    .unwrap_or("unknown");
                Ok(version.to_string())
            }
        }
    }

    fn get_base_url(&self) -> String {
        match self {
            Self::V6(_, config) => config.url.clone(),
            Self::V8(_, config) => config.url.clone(),
        }
    }
}

// 配置管理函数
pub fn get_es_config<'a>(config: &'a AppConfig, name: &str) -> Option<&'a ElasticsearchConfig> {
    config.elasticsearch.iter().find(|c| c.name == name)
}

pub fn list_es_configs(config: &AppConfig) -> &[ElasticsearchConfig] {
    &config.elasticsearch
}

// MCP 工具命令函数
#[tool(
    name = "ESListConfigs",
    description = "List all available Elasticsearch configurations, including name, URL, version, environment, and description for each configuration. This allows automated tools (such as cursor) to enumerate and select the appropriate Elasticsearch cluster for further operations."
)]
pub async fn es_list_configs() -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let configs = list_es_configs(&config);
    let config_list: Vec<String> = configs
        .iter()
        .map(|c| format!(
            "Name: {}, URL: {}, Version: {}, Environment: {}, Description: {}",
            c.name, c.url, c.version, c.environment, c.description
        ))
        .collect();
    Ok(tool_text_content!(config_list.join("\n")))
}

#[tool(
    name = "ESIndexExists",
    description = "Check if a specific Elasticsearch index exists in the given configuration. Takes the configuration name and index name as parameters, and returns a boolean result. Useful for automated tools (such as cursor) to validate index presence before performing further actions."
)]
pub async fn es_index_exists(config_name: String, index: String) -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let es_config = get_es_config(&config, &config_name)
        .ok_or_else(|| anyhow::anyhow!("Configuration not found: {}", config_name))?;

    let es = MCPElasticsearch::new(es_config)?;
    let exists = es.index_exists(&index).await?;
    Ok(tool_text_content!(format!("Index {} exists: {}", index, exists)))
}

#[tool(
    name = "ESGetIndex",
    description = "Retrieve detailed information about a specific Elasticsearch index, including settings, mappings, and metadata, by specifying the configuration and index name. Enables automated tools (such as cursor) to inspect index structure and properties for advanced queries or validation."
)]
pub async fn es_get_index(config_name: String, index: String) -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let es_config = get_es_config(&config, &config_name)
        .ok_or_else(|| anyhow::anyhow!("Configuration not found: {}", config_name))?;

    let es = MCPElasticsearch::new(es_config)?;
    let info = es.get_index(&index).await?;
    Ok(tool_text_content!(serde_json::to_string_pretty(&info)?))
}

#[tool(
    name = "ESGetAliases",
    description = "Get all aliases associated with a specific Elasticsearch index in the given configuration. Returns alias mappings and related metadata, which is useful for automated tools (such as cursor) to resolve index references and manage index routing."
)]
pub async fn es_get_aliases(config_name: String, index: String) -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let es_config = get_es_config(&config, &config_name)
        .ok_or_else(|| anyhow::anyhow!("Configuration not found: {}", config_name))?;

    let es = MCPElasticsearch::new(es_config)?;
    let aliases = es.get_aliases(&index).await?;
    Ok(tool_text_content!(serde_json::to_string_pretty(&aliases)?))
}

#[tool(
    name = "ESGetMapping",
    description = "Fetch the mapping definition for a specific Elasticsearch index, including field types and structure, by configuration and index name. This allows automated tools (such as cursor) to analyze index schemas and validate document compatibility."
)]
pub async fn es_get_mapping(config_name: String, index: String) -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let es_config = get_es_config(&config, &config_name)
        .ok_or_else(|| anyhow::anyhow!("Configuration not found: {}", config_name))?;

    let es = MCPElasticsearch::new(es_config)?;
    let mapping = es.get_mapping(&index).await?;
    Ok(tool_text_content!(serde_json::to_string_pretty(&mapping)?))
}

#[tool(
    name = "ESGetHealth",
    description = "Get the health status of the Elasticsearch cluster for a given configuration. Returns cluster health metrics and status, enabling automated tools (such as cursor) to monitor cluster availability and performance."
)]
pub async fn es_get_health(config_name: String) -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let es_config = get_es_config(&config, &config_name)
        .ok_or_else(|| anyhow::anyhow!("Configuration not found: {}", config_name))?;

    let es = MCPElasticsearch::new(es_config)?;
    let health = es.get_health().await?;
    Ok(tool_text_content!(serde_json::to_string_pretty(&health)?))
}

#[tool(
    name = "ESSearch",
    description = "Execute a search query on a specified Elasticsearch index using the given configuration. Accepts a JSON query string and returns the search results. This is essential for automated tools (such as cursor) to perform dynamic data retrieval and analysis."
)]
pub async fn es_search(config_name: String, index: String, query: String) -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let es_config = get_es_config(&config, &config_name)
        .ok_or_else(|| anyhow::anyhow!("Configuration not found: {}", config_name))?;

    let es = MCPElasticsearch::new(es_config)?;
    let results = es.search(&index, &query).await?;
    Ok(tool_text_content!(serde_json::to_string_pretty(&results)?))
}

#[tool(
    name = "ESGetVersion",
    description = "Retrieve the version information of the Elasticsearch cluster for a given configuration. Returns the version string, which is important for automated tools (such as cursor) to ensure compatibility and select appropriate features."
)]
pub async fn es_get_version(config_name: String) -> Result<ToolResponseContent> {
    let config = crate::utils::nacos_config::get_config_inner()?;
    let es_config = get_es_config(&config, &config_name)
        .ok_or_else(|| anyhow::anyhow!("Configuration not found: {}", config_name))?;

    let es = MCPElasticsearch::new(es_config)?;
    let version_info = es.get_version().await?;
    Ok(tool_text_content!(format!("Version: {}", version_info)))
}
