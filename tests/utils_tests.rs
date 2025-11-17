use pixiv_rs::utils::{download, extract_extension, format_file_size, parse_qs, safe_filename};
use pixiv_rs::network::HttpClient;
use std::path::Path;

#[tokio::test]
async fn test_parse_qs() {
    let url = "https://example.com/api?offset=20&limit=30&tag=test";
    let params = parse_qs(url);
    
    assert_eq!(params.get("offset"), Some(&"20".to_string()));
    assert_eq!(params.get("limit"), Some(&"30".to_string()));
    assert_eq!(params.get("tag"), Some(&"test".to_string()));
}

#[tokio::test]
async fn test_parse_qs_empty() {
    let url = "https://example.com/api";
    let params = parse_qs(url);
    assert!(params.is_empty());
}

#[tokio::test]
async fn test_parse_qs_with_encoded() {
    let url = "https://example.com/api?query=hello%20world";
    let params = parse_qs(url);
    
    assert_eq!(params.get("query"), Some(&"hello world".to_string()));
}

#[tokio::test]
async fn test_format_file_size() {
    assert_eq!(format_file_size(0), "0 B");
    assert_eq!(format_file_size(512), "512.0 B");
    assert_eq!(format_file_size(1024), "1.0 KB");
    assert_eq!(format_file_size(1536), "1.5 KB");
    assert_eq!(format_file_size(1048576), "1.0 MB");
    assert_eq!(format_file_size(1073741824), "1.0 GB");
}

#[tokio::test]
async fn test_safe_filename() {
    assert_eq!(safe_filename("normal.jpg"), "normal.jpg");
    assert_eq!(safe_filename("test/file:name?.jpg"), "test_file_name_.jpg");
    assert_eq!(safe_filename("  spaced  "), "spaced");
    assert_eq!(safe_filename(""), "");
}

#[tokio::test]
async fn test_extract_extension() {
    assert_eq!(extract_extension("https://example.com/image.jpg"), Some("jpg".to_string()));
    assert_eq!(extract_extension("https://example.com/image.png?size=large"), Some("png".to_string()));
    assert_eq!(extract_extension("https://example.com/image"), None);
    assert_eq!(extract_extension("https://example.com/image.JPEG"), Some("jpeg".to_string()));
    // 测试路径中包含点号的情况
    assert_eq!(extract_extension("https://example.com/path.to/image.jpg"), Some("jpg".to_string()));
    assert_eq!(extract_extension("https://example.com/path.to/image"), None);
}

// Note: Download tests need to mock HTTP responses, here we only test function calls
// Actual tests need to mock HTTP client or use test server
#[tokio::test]
async fn test_download_function_exists() {
    // This test only verifies that the function exists and can be called
    // Actual download tests need more complex setup
    let client = HttpClient::new().unwrap();
    let path = Path::new("/tmp/test_image.jpg");
    
    // Since we don't have a real image URL, here we only verify the function signature
    // Actual usage needs to provide a valid URL
    // let result = download(&client, "https://example.com/image.jpg", path).await;
    // assert!(result.is_ok() || result.is_err()); // Only verify that the function can be called
}