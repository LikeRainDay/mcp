use crate::utils::config::{AppConfig, ElasticsearchFieldsConfig};
use anyhow::Result;
use nacos_sdk::api::config::{ConfigChangeListener, ConfigResponse, ConfigServiceBuilder};
use nacos_sdk::api::props::ClientProps;
use nacos_sdk::api::naming::NamingServiceBuilder;
use once_cell::sync::Lazy;
use serde_yaml;
use serde_json;
use std::env;
use std::sync::{Arc, RwLock};
use std::fs;

pub fn nacos_server_addr() -> String {
    env::var("NACOS_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8848".to_string())
}
pub fn nacos_namespace() -> String {
    env::var("NACOS_NAMESPACE").unwrap_or_else(|_| "public".to_string())
}
pub fn nacos_data_id() -> String {
    env::var("NACOS_DATA_ID").unwrap_or_else(|_| "rig-mcp-server.yaml".to_string())
}
pub fn nacos_group() -> String {
    env::var("NACOS_GROUP").unwrap_or_else(|_| "DEFAULT_GROUP".to_string())
}

pub fn elasticsearch_fields_data_id() -> String {
    env::var("ELASTICSEARCH_FIELDS_DATA_ID").unwrap_or_else(|_| "elasticsearch-fields.yml".to_string())
}

// 全局配置缓存
static CONFIG: Lazy<Arc<RwLock<Option<AppConfig>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));
static ES_FIELDS_CONFIG: Lazy<Arc<RwLock<Option<ElasticsearchFieldsConfig>>>> = Lazy::new(|| Arc::new(RwLock::new(None)));

pub fn is_offline_mode() -> bool {
    env::var("OFFLINE_MODE").unwrap_or_else(|_| "false".to_string()) == "true"
}

pub fn get_config_file() -> String {
    env::var("CONFIG_FILE").unwrap_or_else(|_| "config-demo.yml".to_string())
}

pub fn read_config_from_file() -> Result<String> {
    let content = fs::read_to_string(get_config_file())?;
    Ok(content)
}

pub async fn init_nacos_config() -> Result<()> {
    if is_offline_mode() {
        println!("[Config] Running in offline mode, reading from config-demo.yml");
        let content = read_config_from_file()?;
        reload_config_from_str(&content)?;

        // 读取 elasticsearch-fields.yml
        let es_fields_content = fs::read_to_string("elasticsearch-fields.yml")?;
        reload_es_fields_config_from_str(&es_fields_content)?;
        return Ok(());
    }

    let props = ClientProps::new()
        .server_addr(&nacos_server_addr())
        .namespace(&nacos_namespace())
        .app_name("rig-mcp-server");
    let config_service = ConfigServiceBuilder::new(props).build()?;

    // 读取主配置
    let data_id = nacos_data_id();
    let group = nacos_group();
    let config_resp = config_service
        .get_config(data_id.clone(), group.clone())
        .await?;
    reload_config_from_str(config_resp.content())?;

    // 读取 elasticsearch-fields.yml 配置
    let es_fields_data_id = elasticsearch_fields_data_id();
    let es_fields_resp = config_service
        .get_config(es_fields_data_id.clone(), group.clone())
        .await?;
    reload_es_fields_config_from_str(es_fields_resp.content())?;

    // 监听配置变更
    struct Listener;
    impl ConfigChangeListener for Listener {
        fn notify(&self, config_resp: ConfigResponse) {
            let content = config_resp.content();
            if *config_resp.data_id() == nacos_data_id() {
                if let Err(e) = reload_config_from_str(content) {
                    eprintln!("[Nacos] 主配置热加载失败: {e}");
                }
            } else if *config_resp.data_id() == elasticsearch_fields_data_id() {
                if let Err(e) = reload_es_fields_config_from_str(content) {
                    eprintln!("[Nacos] ES字段配置热加载失败: {e}");
                }
            }
        }
    }

    // 添加主配置监听
    config_service
        .add_listener(data_id, group.clone(), Arc::new(Listener))
        .await?;

    // 添加ES字段配置监听
    config_service
        .add_listener(es_fields_data_id, group, Arc::new(Listener))
        .await?;

    Ok(())
}

pub fn reload_config_from_str(content: &str) -> Result<()> {
    // 解析 YAML 配置
    let config: AppConfig = match serde_yaml::from_str(content) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("[Nacos] 配置解析失败: {e}\n原始内容: {}", content);
            return Err(anyhow::anyhow!("配置解析失败: {e}"));
        }
    };
    {
        let mut guard = CONFIG.write().unwrap();
        *guard = Some(config);
    }
    println!("[Nacos] 配置已热加载更新");
    Ok(())
}

pub fn get_config_inner() -> Result<AppConfig> {
    let guard = CONFIG.read().unwrap();
    guard
        .clone()
        .ok_or_else(|| anyhow::anyhow!("配置尚未初始化"))
}

/// 通过namespace和data_id获取nacos配置内容
pub async fn get_nacos_config_by(namespace: &str, data_id: &str, group: &str) -> Result<String> {
    let props = ClientProps::new()
        .server_addr(&nacos_server_addr())
        .namespace(namespace)
        .app_name("rig-mcp-server");
    let config_service = ConfigServiceBuilder::new(props).build()?;
    let config_resp = config_service.get_config(data_id.to_string(), group.to_string()).await?;
    let content = config_resp.content().to_string();
    // 显式drop配置服务
    drop(config_service);
    Ok(content)
}

/// 通过namespace和service_name获取服务实例信息
type ServiceInfo = serde_json::Value;

pub async fn get_nacos_service_info_by(namespace: &str, service_name: &str, group: Option<String>) -> Result<ServiceInfo> {
    let props = ClientProps::new()
        .server_addr(&nacos_server_addr())
        .namespace(namespace)
        .app_name("rig-mcp-server");
    let naming_service = NamingServiceBuilder::new(props).build()?;
    // 获取所有实例（可根据需要调整参数）
    let instances = naming_service.get_all_instances(
        service_name.to_string(),
        group,
        Vec::new(), // clusters
        false, // subscribe
    ).await?;
    // 转为json
    let json = serde_json::to_value(&instances)?;
    // 显式drop命名服务
    drop(naming_service);
    Ok(json)
}

pub fn reload_es_fields_config_from_str(content: &str) -> Result<()> {
    // 解析 YAML 配置
    let config: ElasticsearchFieldsConfig = match serde_yaml::from_str(content) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("[Nacos] ES字段配置解析失败: {e}\n原始内容: {}", content);
            return Err(anyhow::anyhow!("ES字段配置解析失败: {e}"));
        }
    };
    {
        let mut guard = ES_FIELDS_CONFIG.write().unwrap();
        *guard = Some(config);
    }
    println!("[Nacos] ES字段配置已热加载更新");
    Ok(())
}

pub fn get_es_fields_config() -> Result<ElasticsearchFieldsConfig> {
    let guard = ES_FIELDS_CONFIG.read().unwrap();
    guard
        .clone()
        .ok_or_else(|| anyhow::anyhow!("ES字段配置尚未初始化"))
}
