use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;
use mysql::*;
use mysql::prelude::*;
use crate::utils::nacos_config::get_config_inner;
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};

#[tool(
    name = "ExecuteMySQLQuery",
    description = "Execute a MySQL SELECT query and return results. Only SELECT queries are allowed for security reasons.",
    params(
        connection_name = "The name of the MySQL connection to use",
        database = "The database name to use",
        query = "The MySQL SELECT query to execute"
    )
)]
pub async fn execute_mysql_query(connection_name: String, database: String, query: String) -> Result<ToolResponseContent> {
    let query = query.trim();
    if !query.to_lowercase().starts_with("select") {
        return Err(anyhow::anyhow!("Only SELECT queries are allowed for security reasons"));
    }

    let config = get_config_inner()?;

    let mysql_config = config.mysql
        .iter()
        .find(|c| c.name == connection_name)
        .ok_or_else(|| anyhow::anyhow!("MySQL connection '{}' not found", connection_name))?;

    let encoded_password = percent_encode(mysql_config.password.as_bytes(), NON_ALPHANUMERIC).to_string();
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_config.username,
        encoded_password,
        mysql_config.host,
        mysql_config.port,
        database
    );

    let pool = Pool::new(url.as_str())?;

    let result = pool.get_conn()?.query_map(
        query,
        |row: Row| {
            let mut map = std::collections::HashMap::new();
            for (i, col) in row.columns_ref().iter().enumerate() {
                let value: Option<String> = row.get(i);
                map.insert(col.name_str().to_string(), value);
            }
            map
        },
    )?;

    let json = serde_json::to_string_pretty(&result)?;

    drop(pool);

    Ok(tool_text_content!(json))
}

#[tool(
    name = "ListMySQLDatabases",
    description = "List all available databases in a MySQL connection"
)]
pub async fn list_mysql_databases(connection_name: String) -> Result<ToolResponseContent> {
    let config = get_config_inner()?;

    let mysql_config = config.mysql
        .iter()
        .find(|c| c.name == connection_name)
        .ok_or_else(|| anyhow::anyhow!("MySQL connection '{}' not found", connection_name))?;

    let encoded_password = percent_encode(mysql_config.password.as_bytes(), NON_ALPHANUMERIC).to_string();
    let url = format!(
        "mysql://{}:{}@{}:{}",
        mysql_config.username,
        encoded_password,
        mysql_config.host,
        mysql_config.port
    );

    let pool = Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;

    let databases: Vec<String> = conn.query("SHOW DATABASES")?;

    drop(pool);
    Ok(tool_text_content!(databases.join("\n")))
}

#[tool(
    name = "ListMySQLTables",
    description = "List all tables in a specific database"
)]
pub async fn list_mysql_tables(connection_name: String, database: String) -> Result<ToolResponseContent> {
    let config = get_config_inner()?;

    let mysql_config = config.mysql
        .iter()
        .find(|c| c.name == connection_name)
        .ok_or_else(|| anyhow::anyhow!("MySQL connection '{}' not found", connection_name))?;

    let encoded_password = percent_encode(mysql_config.password.as_bytes(), NON_ALPHANUMERIC).to_string();

    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        mysql_config.username,
        encoded_password,
        mysql_config.host,
        mysql_config.port,
        database
    );

    let pool = Pool::new(url.as_str())?;
    let mut conn = pool.get_conn()?;

    let tables: Vec<String> = conn.query("SHOW TABLES")?;

    drop(pool);
    Ok(tool_text_content!(tables.join("\n")))
}

#[tool(
    name = "ListMySQLConnections",
    description = "List all available MySQL connections"
)]
pub async fn list_mysql_connections() -> Result<ToolResponseContent> {
    let config = get_config_inner()?;
    let connections: Vec<_> = config.mysql
        .iter()
        .map(|c| format!(
            "name: {}\ndescription: {}\nhost: {}\nport: {}\nusername: {}\n",
            c.name, c.description, c.host, c.port, c.username
        ))
        .collect();

    Ok(tool_text_content!(connections.join("\n---\n")))
}
