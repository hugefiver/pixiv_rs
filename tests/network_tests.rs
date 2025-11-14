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
    
    // 初始状态应该没有令牌
    assert!(client.access_token().is_none());
    assert!(client.refresh_token().is_none());
    
    // 设置访问令牌
    client.set_access_token("test_access_token".to_string());
    assert_eq!(client.access_token(), Some("test_access_token"));
    
    // 设置刷新令牌
    client.set_refresh_token("test_refresh_token".to_string());
    assert_eq!(client.refresh_token(), Some("test_refresh_token"));
}

#[tokio::test]
async fn test_get_request() {
    let client = HttpClient::new().unwrap();
    
    // 测试一个简单的GET请求
    let result = client.get("https://httpbin.org/get").await;
    
    match result {
        Ok(_) => {
            // 请求成功
        }
        Err(PixivError::NetworkError(_)) => {
            // 网络错误也是可以接受的，可能是网络问题
        }
        Err(e) => {
            panic!("意外的错误类型: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_post_request() {
    let client = HttpClient::new().unwrap();
    
    // 测试一个简单的POST请求
    let data = serde_json::json!({
        "key": "value"
    });
    
    let result = client.post("https://httpbin.org/post", &data).await;
    
    match result {
        Ok(_) => {
            // 请求成功
        }
        Err(PixivError::NetworkError(_)) => {
            // 网络错误也是可以接受的，可能是网络问题
        }
        Err(e) => {
            panic!("意外的错误类型: {:?}", e);
        }
    }
}