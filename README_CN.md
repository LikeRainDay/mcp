# MCP

## 使用方式

- 启动 MCP 服务
```bash
cargo run
```
- 启动 MCP 客户端
```bash
npx @modelcontextprotocol/inspector sse http://127.0.0.1:3001/sse
```
- Run in Docker in Docker

```
docker run -e ALIYUN_ACCESS_KEY_ID=xxx -e ALIYUN_ACCESS_KEY_SECRET=yyy -e ALIYUN_REGION_ID=cn-hangzhou your-image-name
```

- Cursor MCP 配置

[Curosr MCP](https://docs.cursor.com/context/model-context-protocol).

```json
{
  "mcpServers": {
    "server-name": {
      "url": "http://localhost:3001/sse",
    }
  }
}
```
---

## MCP 相关模块说明

本目录下包含与 MCP（管理控制平台）相关的核心功能模块，具体如下：

### 1. mcp_config.rs

- **功能**：负责 MCP 相关的配置信息管理。
- **说明**：定义和加载 MCP 所需的配置信息，便于系统灵活调整参数。

### 2. mcp_time.rs

- **功能**：时间相关的工具函数。
- **说明**：提供时间格式化、时间戳转换等与时间处理相关的实用方法。

### 3. mcp_aliyun_log_cli.rs

- **功能**：阿里云日志 CLI 工具集成。
- **说明**：封装了与阿里云日志服务交互的命令行操作，便于日志的查询与管理。

### 4. mcp_aliyun_cli.rs

- **功能**：阿里云 CLI 工具集成。
- **说明**：封装了与阿里云各类服务交互的命令行操作，支持多种阿里云 API 的调用。

### 5. mod.rs

- **功能**：模块聚合入口。
- **说明**：统一对外暴露本目录下的各个功能模块，便于外部调用。
