use anyhow::Result;
use mcp_core::tool_text_content;
use mcp_core::types::ToolResponseContent;
use mcp_core_macros::tool;
use crate::utils::date_util::now_datetime_string;

#[tool(
    name = "GetCurrentTime",
    description = "Get current server time in YYYY-MM-DD HH:MM:SS format, no parameters required",
)]
pub async fn get_current_time() -> Result<ToolResponseContent> {
    let now = now_datetime_string();
    Ok(tool_text_content!(now))
}
