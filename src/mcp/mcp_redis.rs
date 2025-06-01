use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;
use redis::{Client};
use crate::utils::nacos_config::get_config_inner;

const RESTRICTED_COMMANDS: &[&str] = &[
    "DEL", "FLUSHDB", "FLUSHALL", "RENAME", "RENAMENX",
    "SET", "SETEX", "SETNX", "MSET", "MSETNX",
    "HDEL", "HMSET", "HSET", "HSETNX",
    "LREM", "LPUSH", "RPUSH", "LPOP", "RPOP",
    "SADD", "SREM", "SPOP", "SINTERSTORE", "SUNIONSTORE",
    "ZADD", "ZREM", "ZREMRANGEBYSCORE", "ZREMRANGEBYRANK",
    "EXPIRE", "EXPIREAT", "PEXPIRE", "PEXPIREAT",
    "PERSIST", "MOVE", "SELECT", "SWAPDB",
];

#[tool(
    name = "ExecuteRedisCommand",
    description = "Execute a Redis command and return results. Write operations are not allowed in production environment.",
    params(
        connection_name = "The name of the Redis connection to use",
        database = "The database number to use",
        command = "The Redis command to execute (e.g., GET key, HGETALL hash)"
    )
)]
pub async fn execute_redis_command(connection_name: String, database: i32, command: String) -> Result<ToolResponseContent> {
    let config = get_config_inner()?;

    let redis_config = config.redis
        .iter()
        .find(|c| c.name == connection_name)
        .ok_or_else(|| anyhow::anyhow!("Redis connection '{}' not found", connection_name))?;

    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return Err(anyhow::anyhow!("Empty command"));
    }

    let cmd = parts[0].to_uppercase();

    if redis_config.environment.to_lowercase() == "prod" && RESTRICTED_COMMANDS.contains(&cmd.as_str()) {
        return Err(anyhow::anyhow!(
            "Command '{}' is not allowed in production environment",
            cmd
        ));
    }

    let client = Client::open(format!(
        "redis://{}:{}/0",
        redis_config.host,
        redis_config.port
    ))?;

    let mut con = client.get_connection()?;
    if let Some(password) = &redis_config.password {
        redis::cmd("AUTH").arg(password).execute(&mut con);
    }

    if database != 0 {
        let select_result: redis::RedisResult<()> = redis::cmd("SELECT").arg(database).query(&mut con);
        if let Err(e) = select_result {
            return Err(anyhow::anyhow!("Redis select db error: {}", e));
        }
    }

    // 执行命令，兼容多种返回类型
    let result_string: Option<String> = match redis::cmd(&cmd).arg(&parts[1..]).query::<String>(&mut con) {
        Ok(s) => Some(s),
        Err(_) => None,
    };
    if let Some(s) = result_string {
        return Ok(tool_text_content!(s));
    }

    let result_vec: Option<Vec<String>> = match redis::cmd(&cmd).arg(&parts[1..]).query::<Vec<String>>(&mut con) {
        Ok(v) => Some(v),
        Err(_) => None,
    };
    if let Some(v) = result_vec {
        return Ok(tool_text_content!(v.join("\n")));
    }

    let result_i64: Option<i64> = match redis::cmd(&cmd).arg(&parts[1..]).query::<i64>(&mut con) {
        Ok(i) => Some(i),
        Err(_) => None,
    };
    if let Some(i) = result_i64 {
        return Ok(tool_text_content!(i.to_string()));
    }

    let result_f64: Option<f64> = match redis::cmd(&cmd).arg(&parts[1..]).query::<f64>(&mut con) {
        Ok(f) => Some(f),
        Err(_) => None,
    };
    if let Some(f) = result_f64 {
        return Ok(tool_text_content!(f.to_string()));
    }

    let result_bool: Option<bool> = match redis::cmd(&cmd).arg(&parts[1..]).query::<bool>(&mut con) {
        Ok(b) => Some(b),
        Err(_) => None,
    };
    if let Some(b) = result_bool {
        return Ok(tool_text_content!(b.to_string()));
    }

    // 最后用 Debug 格式兜底
    let result_any = redis::cmd(&cmd).arg(&parts[1..]).query::<redis::Value>(&mut con);
    match result_any {
        Ok(val) => Ok(tool_text_content!(format!("{:?}", val))),
        Err(e) => Err(anyhow::anyhow!("Redis command error: {}", e)),
    }
}

#[tool(
    name = "ListRedisDatabases",
    description = "List all available databases in a Redis connection"
)]
pub async fn list_redis_databases(connection_name: String) -> Result<ToolResponseContent> {
    let config = get_config_inner()?;

    let redis_config = config.redis
        .iter()
        .find(|c| c.name == connection_name)
        .ok_or_else(|| anyhow::anyhow!("Redis connection '{}' not found", connection_name))?;

    let client = Client::open(format!(
        "redis://{}:{}/0",
        redis_config.host,
        redis_config.port
    ))?;

    let mut con = client.get_connection()?;
    if let Some(password) = &redis_config.password {
        redis::cmd("AUTH").arg(password).execute(&mut con);
    }

    // Get the number of databases
    let config: Vec<String> = redis::cmd("CONFIG").arg("GET").arg("databases").query(&mut con)?;
    if config.len() != 2 {
        return Err(anyhow::anyhow!("Unexpected CONFIG GET databases response: {:?}", config));
    }
    let db_count: i32 = config[1].parse()?;

    // Create a list of database numbers
    let databases: Vec<String> = (0..db_count).map(|i| i.to_string()).collect();

    Ok(tool_text_content!(databases.join("\n")))
}

#[tool(
    name = "ListRedisConnections",
    description = "List all available Redis connections"
)]
pub async fn list_redis_connections() -> Result<ToolResponseContent> {
    let config = get_config_inner()?;
    let connections: Vec<_> = config.redis
        .iter()
        .map(|c| format!(
            "{}: {} ({}) [{}]",
            c.name,
            c.description,
            c.host,
            c.environment
        ))
        .collect();

    Ok(tool_text_content!(connections.join("\n")))
}
