use chrono::Local;

/// 获取当前日期时间，格式为 "YYYY-MM-DD HH:MM:SS"
pub fn now_datetime_string() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}
