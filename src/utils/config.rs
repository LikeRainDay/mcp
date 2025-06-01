use serde::{Deserialize, Serialize};

/// 应用主配置
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SlsIndex {
    pub name: String,         // 索引名
    pub description: String,  // 索引说明
    pub index_type: String,         // 索引类型
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SlsLogstore {
    pub name: String,         // logstore 名称
    pub description: String,  // logstore 说明
    pub indexes: Vec<SlsIndex>, // 该 logstore 下的索引
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SlsProject {
    pub name: String,         // project 名称
    pub environment: String,  // 环境说明（如 prod、test、dev）
    pub description: String,  // project 说明
    pub logstores: Vec<SlsLogstore>, // 该 project 下的 logstore
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct SlsConfig {
    pub projects: Vec<SlsProject>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NacosConfig {
    pub description: String, // nacos 配置说明
    pub namespace: String,
    pub data_ids: Vec<NacosDataId>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct NacosDataId {
    pub data_id: String,
    pub application_name: String,
    pub group: String,
    pub description: String, // nacos 数据 id 说明
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct MySQLConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub description: String,
    pub name: String,  // 连接名称，用于标识不同的连接
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RedisConfig {
    pub name: String,        // 连接名称
    pub host: String,        // Redis 主机
    pub port: u16,          // Redis 端口
    pub password: Option<String>, // Redis 密码（可选）
    pub description: String, // 连接描述
    pub environment: String, // 环境（如 "prod", "staging", "dev"）
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct AppConfig {
    pub sls: SlsConfig,
    pub nacos: Vec<NacosConfig>,
    pub mysql: Vec<MySQLConfig>,  // 改为 Vec 以支持多个连接
    pub redis: Vec<RedisConfig>,  // Redis 连接配置
}
