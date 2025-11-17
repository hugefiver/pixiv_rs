use pixiv_rs::client::bypass_sni::BypassSniAppClient;
use pixiv_rs::error::PixivError;

#[tokio::test]
async fn test_bypass_sni_client_creation() {
    // Test creating client with valid IP address
    let result = BypassSniAppClient::with_ip("210.140.131.145");
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_invalid_ip() {
    // Test creating client with invalid IP address
    let result = BypassSniAppClient::with_ip("invalid_ip");
    assert!(result.is_err());
    
    // Check error type
    match result {
        Err(PixivError::NetworkError(_)) => (),
        _ => panic!("Expected NetworkError"),
    }
}