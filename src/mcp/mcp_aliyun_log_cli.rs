// 日志服务CLI https://help.aliyun.com/zh/sls/developer-reference/overview-of-log-service-cli?spm=a2c4g.11186623.help-menu-28958.d_3_5_0.474d164aLcJeyg&scm=20140722.H_93539._.OR_help-T_cn~zh-V_1

use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;

#[tool(
    name = "RunAliyunLogCliCommand",
    description = "Execute aliyunlog CLI log subcommand and return results. If unsure about what to execute, first call the GetConfig MCP tool to get configuration information. If time is involved, first call GetCurrentTime to get the current time. Example: {\"command\":\"log get_log_all --project=xxx --logstore=xxx --from_time=2024-06-01 --to_time=2024-06-02\"}",
    params(command = "The aliyunlog CLI log subcommand string to execute (without aliyunlog prefix)")
)]
pub async fn run_aliyun_log_cli_command(command: String) -> Result<ToolResponseContent> {
    use std::process::Command as SysCommand;
    let args: Vec<&str> = command.split_whitespace().collect();
    let cli_output = SysCommand::new("aliyunlog").args(&args).output();
    let cli_result = match cli_output {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            format!(
                "[aliyunlog cli stdout]:\n{}\n[aliyunlog cli stderr]:\n{}",
                stdout, stderr
            )
        }
        Err(e) => format!("[aliyunlog cli error]: {}", e),
    };
    Ok(tool_text_content!(cli_result))
}
