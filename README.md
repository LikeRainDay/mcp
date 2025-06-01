# MCP

[中文说明](./README_CN.md)

## Usage

- Start MCP server
```bash
cargo run
```
- Start MCP client
```bash
npx @modelcontextprotocol/inspector sse http://127.0.0.1:3001/sse
```
- Run in Docker in Docker

```
docker run -e ALIYUN_ACCESS_KEY_ID=xxx -e ALIYUN_ACCESS_KEY_SECRET=yyy -e ALIYUN_REGION_ID=cn-hangzhou your-image-name
```

- Cursor MCP Configuration

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

## MCP Module Descriptions

This directory contains the core functional modules related to MCP (Management Control Platform), as follows:

### 1. mcp_config.rs

- **Function**: Responsible for managing MCP-related configuration information.
- **Description**: Defines and loads the configuration information required by MCP, allowing flexible adjustment of system parameters.

### 2. mcp_time.rs

- **Function**: Utility functions related to time.
- **Description**: Provides practical methods for time formatting, timestamp conversion, and other time-related processing.

### 3. mcp_aliyun_log_cli.rs

- **Function**: Aliyun Log CLI tool integration.
- **Description**: Encapsulates command-line operations for interacting with Aliyun Log Service, facilitating log query and management.

### 4. mcp_aliyun_cli.rs

- **Function**: Aliyun CLI tool integration.
- **Description**: Encapsulates command-line operations for interacting with various Aliyun services, supporting multiple Aliyun API calls.

### 5. mod.rs

- **Function**: Module aggregation entry point.
- **Description**: Uniformly exposes the functional modules in this directory for external calls.
