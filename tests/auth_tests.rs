use pixiv_rs::auth::AuthClient;
use pixiv_rs::error::PixivError;

#[test]
fn test_auth_client_creation() {
    let result = AuthClient::new();
    assert!(result.is_ok());
}

#[test]
fn test_security_headers_generation() {
    let client = AuthClient::new().unwrap();
    let headers = client.client().generate_security_headers();
    
    assert!(headers.contains_key("x-client-time"));
    assert!(headers.contains_key("x-client-hash"));
    assert!(!headers.get("x-client-time").unwrap().is_empty());
    assert!(!headers.get("x-client-hash").unwrap().is_empty());
}

#[test]
fn test_token_expiry_check() {
    use chrono::Utc;
    use pixiv_rs::auth::{AuthResponse, User, ProfileImageUrls};
    
    let mut auth_response = AuthResponse {
        access_token: "test_token".to_string(),
        refresh_token: "refresh_token".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 3600, // 1小时
        user: User {
            id: 12345,
            name: "Test User".to_string(),
            account: "testuser".to_string(),
            email: None,
            profile_image_urls: ProfileImageUrls {
                px_16x16: None,
                px_50x50: None,
                px_170x170: None,
            },
        },
        obtained_at: Utc::now(),
    };

    let auth_client = AuthClient::new().unwrap();
    
    // 新令牌不应该过期
    assert!(!auth_client.is_token_expired(&auth_response));
    
    // 设置令牌为过去时间
    auth_response.obtained_at = Utc::now() - chrono::Duration::hours(2);
    
    // 过期令牌应该被检测到
    assert!(auth_client.is_token_expired(&auth_response));
}

#[tokio::test]
async fn test_login_with_invalid_credentials() {
    let mut auth_client = AuthClient::new().unwrap();
    
    // Use invalid username and password
    let result = auth_client.login("invalid_user", "invalid_password").await;
    
    // Should return authentication error
    match result {
        Err(PixivError::AuthError(_)) => {
            // This is the expected result
        }
        Err(e) => {
            panic!("Expected authentication error, but got: {:?}", e);
        }
        Ok(_) => {
            panic!("Expected authentication error, but login succeeded");
        }
    }
}

#[tokio::test]
async fn test_refresh_with_invalid_token() {
    let mut auth_client = AuthClient::new().unwrap();
    
    // Use invalid refresh token
    let result = auth_client.refresh_access_token("invalid_refresh_token").await;
    
    // Should return authentication error
    match result {
        Err(PixivError::AuthError(_)) => {
            // This is the expected result
        }
        Err(e) => {
            panic!("Expected authentication error, but got: {:?}", e);
        }
        Ok(_) => {
            panic!("Expected authentication error, but refresh succeeded");
        }
    }
}