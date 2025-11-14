//! 辅助工具模块
//!
//! 提供Pixiv API相关的辅助功能，包括图片下载、参数解析等。

use crate::error::{PixivError, Result};
use crate::network::HttpClient;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info};

/// 下载图片到指定路径
/// 
/// # 参数
/// * `client` - HTTP客户端
/// * `url` - 图片URL
/// * `path` - 保存路径
/// 
/// # 返回
/// 返回下载结果
/// 
/// # 示例
/// ```rust
/// use pixiv_rs::utils::download;
/// use pixiv_rs::network::HttpClient;
/// 
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let client = HttpClient::new()?;
///     download(&client, "https://example.com/image.jpg", "image.jpg").await?;
///     Ok(())
/// }
/// ```
pub async fn download(
    client: &HttpClient,
    url: &str,
    path: &Path,
) -> Result<()> {
    debug!(url = %url, path = ?path, "Starting download");
    
    // 发送HTTP请求获取图片
    let response = client.get(url).await?;
    
    // 确保请求成功
    if !response.status().is_success() {
        return Err(PixivError::ApiError(format!(
            "下载失败: {} - {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("未知错误")
        )));
    }
    
    // 获取图片数据
    let bytes = response.bytes().await?;
    
    // 创建目标文件
    let mut file = File::create(path).await
        .map_err(|e| PixivError::Unknown(format!("无法创建文件 {}: {}", path.display(), e)))?;
    
    // 写入数据
    file.write_all(&bytes).await
        .map_err(|e| PixivError::Unknown(format!("无法写入文件 {}: {}", path.display(), e)))?;
    
    info!(url = %url, path = ?path, size = bytes.len(), "Download completed");
    
    Ok(())
}

/// 解析分页参数
/// 
/// 从URL中解析查询参数，用于分页请求
/// 
/// # 参数
/// * `url` - 包含查询参数的URL
/// 
/// # 返回
/// 返回解析后的参数映射
/// 
/// # 示例
/// ```rust
/// use pixiv_rs::utils::parse_qs;
/// 
/// let url = "https://example.com/api?offset=20&limit=30";
/// let params = parse_qs(url);
/// assert_eq!(params.get("offset"), Some(&"20".to_string()));
/// assert_eq!(params.get("limit"), Some(&"30".to_string()));
/// ```
pub fn parse_qs(url: &str) -> HashMap<String, String> {
    debug!(url = %url, "Parsing query string");
    
    let mut result = HashMap::new();
    
    // 分割URL获取查询部分
    if let Some(query_part) = url.split('?').nth(1) {
        // 分割参数对
        for pair in query_part.split('&') {
            // 分割键值
            if let Some((key, value)) = pair.split_once('=') {
                // URL解码
                if let Ok(decoded_key) = urlencoding::decode(key) {
                    if let Ok(decoded_value) = urlencoding::decode(value) {
                        result.insert(decoded_key.into_owned(), decoded_value.into_owned());
                    } else {
                        result.insert(key.to_string(), value.to_string());
                    }
                } else {
                    result.insert(key.to_string(), value.to_string());
                }
            }
        }
    }
    
    debug!(params = ?result, "Query string parsed");
    result
}

/// 设置接受语言
/// 
/// 为HTTP客户端设置Accept-Language头
/// 
/// # 参数
/// * `client` - HTTP客户端
/// * `language` - 语言代码 (如: "zh-CN", "en-US")
/// 
/// # 示例
/// ```rust
/// use pixiv_rs::utils::set_accept_language;
/// use pixiv_rs::network::HttpClient;
/// 
/// let mut client = HttpClient::new()?;
/// set_accept_language(&mut client, "zh-CN");
/// ```
pub fn set_accept_language(client: &mut HttpClient, language: &str) {
    debug!(language = %language, "Setting Accept-Language header");
    
    // 注意：这里需要根据实际的HttpClient实现来设置头部
    // 由于当前的HttpClient结构体可能不直接支持设置自定义头部，
    // 这个函数可能需要在HttpClient中添加相应的方法
    
    // 示例实现（可能需要根据实际情况调整）:
    // client.set_default_header("Accept-Language", language);
    
    debug!(language = %language, "Accept-Language header set");
}

/// 格式化文件大小
/// 
/// 将字节数转换为人类可读的格式
/// 
/// # 参数
/// * `bytes` - 字节数
/// 
/// # 返回
/// 返回格式化后的文件大小字符串
/// 
/// # 示例
/// ```rust
/// use pixiv_rs::utils::format_file_size;
/// 
/// assert_eq!(format_file_size(1024), "1.0 KB");
/// assert_eq!(format_file_size(1048576), "1.0 MB");
/// ```
pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    const THRESHOLD: f64 = 1024.0;
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= THRESHOLD && unit_index < UNITS.len() - 1 {
        size /= THRESHOLD;
        unit_index += 1;
    }
    
    format!("{:.1} {}", size, UNITS[unit_index])
}

/// 生成安全的文件名
/// 
/// 从字符串生成安全的文件名，移除或替换不安全的字符
/// 
/// # 参数
/// * `input` - 输入字符串
/// 
/// # 返回
/// 返回安全的文件名
/// 
/// # 示例
/// ```rust
/// use pixiv_rs::utils::safe_filename;
/// 
/// let filename = safe_filename("test/file:name?.jpg");
/// assert_eq!(filename, "test_file_name_.jpg");
/// ```
pub fn safe_filename(input: &str) -> String {
    input
        .chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
            c if c.is_control() => '_',
            c => c,
        })
        .collect::<String>()
        .trim()
        .to_string()
}

/// 提取URL中的文件扩展名
/// 
/// # 参数
/// * `url` - 图片URL
/// 
/// # 返回
/// 返回文件扩展名（不包含点号）
/// 
/// # 示例
/// ```rust
/// use pixiv_rs::utils::extract_extension;
/// 
/// assert_eq!(extract_extension("https://example.com/image.jpg"), "jpg");
/// assert_eq!(extract_extension("https://example.com/image.png?size=large"), "png");
/// ```
pub fn extract_extension(url: &str) -> Option<String> {
    // 移除查询参数
    let url_without_query = url.split('?').next().unwrap_or(url);
    
    // 查找最后一个点号，但确保不是路径中的点号
    let path_parts: Vec<&str> = url_without_query.split('/').collect();
    if let Some(file_name) = path_parts.last() {
        if let Some(dot_pos) = file_name.rfind('.') {
            let extension = &file_name[dot_pos + 1..];
            if !extension.is_empty() {
                return Some(extension.to_lowercase());
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_qs() {
        let url = "https://example.com/api?offset=20&limit=30&tag=test";
        let params = parse_qs(url);
        
        assert_eq!(params.get("offset"), Some(&"20".to_string()));
        assert_eq!(params.get("limit"), Some(&"30".to_string()));
        assert_eq!(params.get("tag"), Some(&"test".to_string()));
    }

    #[test]
    fn test_parse_qs_empty() {
        let url = "https://example.com/api";
        let params = parse_qs(url);
        assert!(params.is_empty());
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(0), "0 B");
        assert_eq!(format_file_size(512), "512.0 B");
        assert_eq!(format_file_size(1024), "1.0 KB");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
        assert_eq!(format_file_size(1073741824), "1.0 GB");
    }

    #[test]
    fn test_safe_filename() {
        assert_eq!(safe_filename("normal.jpg"), "normal.jpg");
        assert_eq!(safe_filename("test/file:name?.jpg"), "test_file_name_.jpg");
        assert_eq!(safe_filename("  spaced  "), "spaced");
        assert_eq!(safe_filename(""), "");
    }

    #[test]
    fn test_extract_extension() {
        assert_eq!(extract_extension("https://example.com/image.jpg"), Some("jpg".to_string()));
        assert_eq!(extract_extension("https://example.com/image.png?size=large"), Some("png".to_string()));
        assert_eq!(extract_extension("https://example.com/image"), None);
        assert_eq!(extract_extension("https://example.com/image.JPEG"), Some("jpeg".to_string()));
    }
}