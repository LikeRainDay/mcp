use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;

// https://next.api.aliyun.com/api/Sls/2020-12-30/CreateProject?spm=a2c4g.11186623.0.0.23417a4fwfdbTI&lang=JAVA&sdkStyle=dara&params=%7B%7D&RegionId=cn-beijing&tab=CLI
// https://help.aliyun.com/zh/sls/developer-reference/api-sls-2020-12-30-listlogstores?spm=a2c4g.11186623.help-menu-28958.d_3_2_3_1_10.22b35b8cXg6GRV
#[tool(
    name = "RunAliyunCliCommand",
    description = "Execute any aliyun CLI command and return results. Example: {\"command\":\"sls GetLogs --ProjectName ack-test-cluster-log --LogstoreName user-center-api --from 1718000000 --to 1718003600\"}",
    params(command = "The aliyun CLI command string to execute (without aliyun prefix)")
)]
pub async fn run_aliyun_cli_command(command: String) -> Result<ToolResponseContent> {
    use std::process::Command as SysCommand;
    let args: Vec<&str> = command.split_whitespace().collect();
    let cli_output = SysCommand::new("aliyun").args(&args).output();
    let cli_result = match cli_output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!(
                "[aliyun cli stdout]:\n{}\n[aliyun cli stderr]:\n{}",
                stdout, stderr
            )
        }
        Err(e) => format!("[aliyun cli error]: {}", e),
    };
    Ok(tool_text_content!(cli_result))
}
