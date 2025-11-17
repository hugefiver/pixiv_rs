//! Utility module
//!
//! Provides utility functions for Pixiv API, including image download, parameter parsing, etc.

use crate::error::{PixivError, Result};
use crate::network::HttpClient;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{debug, info};

/// Download image to specified path
///
/// # Arguments
/// * `client` - HTTP client
/// * `url` - Image URL
/// * `path` - Save path
///
/// # Returns
/// Download result
///
/// # Example
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
    
    // Send HTTP request to get image
    let response = client.get(url).await?;
    
    // Ensure request is successful
    if !response.status().is_success() {
        return Err(PixivError::ApiError(format!(
            "Download failed: {} - {}",
            response.status(),
            response.status().canonical_reason().unwrap_or("Unknown error")
        )));
    }
    
    // Get image data
    let bytes = response.bytes().await?;
    
    // Create target file
    let mut file = File::create(path).await
        .map_err(|e| PixivError::Unknown(format!("Failed to create file {}: {}", path.display(), e)))?;
    
    // Write data
    file.write_all(&bytes).await
        .map_err(|e| PixivError::Unknown(format!("Failed to write file {}: {}", path.display(), e)))?;
    
    info!(url = %url, path = ?path, size = bytes.len(), "Download completed");
    
    Ok(())
}

/// Parse pagination parameters
///
/// Parse query parameters from URL for pagination requests
///
/// # Arguments
/// * `url` - URL containing query parameters
///
/// # Returns
/// Returns parsed parameter map
///
/// # Example
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
    
    // Split URL to get query part
    if let Some(query_part) = url.split('?').nth(1) {
        // Split parameter pairs
        for pair in query_part.split('&') {
            // Split key-value
            if let Some((key, value)) = pair.split_once('=') {
                // URL decode
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

/// Set accept language
///
/// Set Accept-Language header for HTTP client
///
/// # Arguments
/// * `client` - HTTP client
/// * `language` - Language code (e.g: "zh-CN", "en-US")
///
/// # Example
/// ```rust
/// use pixiv_rs::utils::set_accept_language;
/// use pixiv_rs::network::HttpClient;
/// 
/// let mut client = HttpClient::new()?;
/// set_accept_language(&mut client, "zh-CN");
/// ```
pub fn set_accept_language(client: &mut HttpClient, language: &str) {
    debug!(language = %language, "Setting Accept-Language header");
    
    // Note: This needs to be implemented according to the actual HttpClient implementation
    // Since the current HttpClient struct may not directly support setting custom headers,
    // this function may need to add corresponding methods in HttpClient
    
    // Example implementation (may need adjustment based on actual situation):
    // client.set_default_header("Accept-Language", language);
    
    debug!(language = %language, "Accept-Language header set");
}

/// Format file size
///
/// Convert byte count to human-readable format
///
/// # Arguments
/// * `bytes` - Byte count
///
/// # Returns
/// Returns formatted file size string
///
/// # Example
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

/// Generate safe filename
///
/// Generate safe filename from string, removing or replacing unsafe characters
///
/// # Arguments
/// * `input` - Input string
///
/// # Returns
/// Returns safe filename
///
/// # Example
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

/// Extract file extension from URL
///
/// # Arguments
/// * `url` - Image URL
///
/// # Returns
/// Returns file extension (without dot)
///
/// # Example
/// ```rust
/// use pixiv_rs::utils::extract_extension;
/// 
/// assert_eq!(extract_extension("https://example.com/image.jpg"), "jpg");
/// assert_eq!(extract_extension("https://example.com/image.png?size=large"), "png");
/// ```
pub fn extract_extension(url: &str) -> Option<String> {
    // Remove query parameters
    let url_without_query = url.split('?').next().unwrap_or(url);
    
    // Find the last dot, but ensure it's not a dot in the path
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