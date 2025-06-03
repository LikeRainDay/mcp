use anyhow::Result;
use mcp_core::types::ClientCapabilities;
use mcp_core::types::Implementation;
use mcp_core::{
    client::ClientBuilder, server::Server, transport::ServerSseTransport, types::ServerCapabilities,
};
use serde_json::json;
mod mcp;
mod utils;
use crate::mcp::mcp_aliyun_cli::RunAliyunCliCommand;
use crate::mcp::mcp_aliyun_log_cli::RunAliyunLogCliCommand;
use crate::mcp::mcp_config::{GetConfig, GetNacosConfig, GetSlsConfig};
use crate::mcp::mcp_mysql::{
    ExecuteMysqlQuery, ListMysqlConnections, ListMysqlDatabases, ListMysqlTables,
};
use crate::mcp::mcp_nacos::{GetNacosConfigByClient, GetNacosServiceInfoByClient};
use crate::mcp::mcp_redis::{ExecuteRedisCommand, ListRedisConnections, ListRedisDatabases};

use crate::mcp::mcp_elasticsearch::{
    EsGetAliases, EsGetHealth, EsGetIndex, EsGetMapping, EsGetVersion, EsIndexExists,
    EsListConfigs, EsSearch,
};
use crate::mcp::mcp_elasticsearch_fields::{GetEsFieldsConfig, GetEsIndexFields};

use crate::mcp::mcp_time::GetCurrentTime;
use crate::utils::nacos_config::init_nacos_config;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();
    init_nacos_config().await?;

    let mcp_server_protocol = Server::builder("add".to_string(), "1.0".to_string())
        .capabilities(ServerCapabilities {
            tools: Some(json!({
                "listChanged": false,
            })),
            ..Default::default()
        })
        .register_tool(RunAliyunCliCommand::tool(), RunAliyunCliCommand::call())
        .register_tool(
            RunAliyunLogCliCommand::tool(),
            RunAliyunLogCliCommand::call(),
        )
        .register_tool(GetCurrentTime::tool(), GetCurrentTime::call())
        .register_tool(GetConfig::tool(), GetConfig::call())
        .register_tool(
            GetNacosConfigByClient::tool(),
            GetNacosConfigByClient::call(),
        )
        .register_tool(
            GetNacosServiceInfoByClient::tool(),
            GetNacosServiceInfoByClient::call(),
        )
        .register_tool(GetSlsConfig::tool(), GetSlsConfig::call())
        .register_tool(GetNacosConfig::tool(), GetNacosConfig::call())
        .register_tool(ExecuteMysqlQuery::tool(), ExecuteMysqlQuery::call())
        .register_tool(ListMysqlConnections::tool(), ListMysqlConnections::call())
        .register_tool(ExecuteRedisCommand::tool(), ExecuteRedisCommand::call())
        .register_tool(ListRedisConnections::tool(), ListRedisConnections::call())
        .register_tool(ListMysqlDatabases::tool(), ListMysqlDatabases::call())
        .register_tool(ListMysqlTables::tool(), ListMysqlTables::call())
        .register_tool(ListRedisDatabases::tool(), ListRedisDatabases::call())
        .register_tool(EsListConfigs::tool(), EsListConfigs::call())
        .register_tool(EsIndexExists::tool(), EsIndexExists::call())
        .register_tool(EsGetIndex::tool(), EsGetIndex::call())
        .register_tool(EsGetAliases::tool(), EsGetAliases::call())
        .register_tool(EsGetMapping::tool(), EsGetMapping::call())
        .register_tool(EsGetHealth::tool(), EsGetHealth::call())
        .register_tool(EsSearch::tool(), EsSearch::call())
        .register_tool(EsGetVersion::tool(), EsGetVersion::call())
        .register_tool(GetEsFieldsConfig::tool(), GetEsFieldsConfig::call())
        .register_tool(GetEsIndexFields::tool(), GetEsIndexFields::call())
        .build();

    let mcp_server_transport =
        ServerSseTransport::new("127.0.0.1".to_string(), 3001, mcp_server_protocol);

    let _ = Server::start(mcp_server_transport.clone()).await;

    let mcp_client = ClientBuilder::new(mcp_server_transport).build();

    let _ = mcp_client.open().await;
    let init_res = mcp_client
        .initialize(
            Implementation {
                name: "mcp-client".to_string(),
                version: "1.0.0".to_string(),
            },
            ClientCapabilities::default(),
        )
        .await;
    println!("Initialized: {:?}", init_res);

    let tools_list_res = mcp_client.list_tools(None, None).await;
    println!("Tools: {:?}", tools_list_res);
    Ok(())
}
