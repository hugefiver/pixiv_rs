use pixiv_rs::client::bypass_sni::BypassSniAppClient;
use pixiv_rs::error::PixivError;

#[tokio::test]
async fn test_bypass_sni_client_creation() {
    // 测试使用有效的IP地址创建客户端
    let result = BypassSniAppClient::with_ip("210.140.131.145");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_invalid_ip() {
    // 测试使用无效的IP地址创建客户端
    let result = BypassSniAppClient::with_ip("invalid_ip");
    assert!(result.is_err());
    
    // 检查错误类型
    match result {
        Err(PixivError::NetworkError(_)) => (),
        _ => panic!("Expected NetworkError"),
    }
}