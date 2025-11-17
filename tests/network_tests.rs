use pixiv_rs::network::HttpClient;
use pixiv_rs::error::PixivError;

#[test]
fn test_http_client_creation() {
    let result = HttpClient::new();
    assert!(result.is_ok());
}

#[test]
fn test_security_headers_generation() {
    let client = HttpClient::new().unwrap();
    let headers = client.generate_security_headers();
    
    assert!(headers.contains_key("x-client-time"));
    assert!(headers.contains_key("x-client-hash"));
    assert!(!headers.get("x-client-time").unwrap().is_empty());
    assert!(!headers.get("x-client-hash").unwrap().is_empty());
}

#[test]
fn test_token_management() {
    let mut client = HttpClient::new().unwrap();
    
    // Initial state should have no tokens
    assert!(client.access_token().is_none());
    assert!(client.refresh_token().is_none());
    
    // Set access token
    client.set_access_token("test_access_token".to_string());
    assert_eq!(client.access_token(), Some("test_access_token"));
    
    // Set refresh token
    client.set_refresh_token("test_refresh_token".to_string());
    assert_eq!(client.refresh_token(), Some("test_refresh_token"));
}

#[tokio::test]
async fn test_get_request() {
    let client = HttpClient::new().unwrap();
    
    // Test a simple GET request
    let result = client.get("https://httpbin.org/get").await;
    
    match result {
        Ok(_) => {
            // Request successful
        }
        Err(PixivError::NetworkError(_)) => {
            // Network error is also acceptable, might be network issues
        }
        Err(e) => {
            panic!("Unexpected error type: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_post_request() {
    let client = HttpClient::new().unwrap();
    
    // Test a simple POST request
    let data = serde_json::json!({
        "key": "value"
    });
    
    let result = client.post("https://httpbin.org/post", &data).await;
    
    match result {
        Ok(_) => {
            // Request successful
        }
        Err(PixivError::NetworkError(_)) => {
            // Network error is also acceptable, might be network issues
        }
        Err(e) => {
            panic!("Unexpected error type: {:?}", e);
        }
    }
}