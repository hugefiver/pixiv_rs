use crate::error::{PixivError, Result};
use crate::network::HttpClient;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Pixiv API authentication client
#[derive(Debug, Clone)]
pub struct AuthClient {
    /// HTTP client
    client: HttpClient,
    /// Authentication base URL
    auth_url: String,
}

/// Authentication response data
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Access token
    pub access_token: String,
    /// Refresh token
    pub refresh_token: String,
    /// Token type
    pub token_type: String,
    /// Expiration time (seconds)
    pub expires_in: u64,
    /// User information
    pub user: User,
    /// Token acquisition time
    #[serde(skip)]
    pub obtained_at: DateTime<Utc>,
}

/// User information
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// User ID
    pub id: u64,
    /// Username
    pub name: String,
    /// Account name
    pub account: String,
    /// Email
    pub email: Option<String>,
    /// Avatar URL
    pub profile_image_urls: ProfileImageUrls,
}

/// User avatar URL
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileImageUrls {
    /// Small avatar
    pub px_16x16: Option<String>,
    /// Medium avatar
    pub px_50x50: Option<String>,
    /// Large avatar
    pub px_170x170: Option<String>,
}

impl AuthClient {
    /// Create new authentication client
    pub fn new() -> Result<Self> {
        let client = HttpClient::new()?;
        Ok(Self {
            client,
            auth_url: "https://oauth.secure.pixiv.net/auth/token".to_string(),
        })
    }

    /// Login with username and password
    pub async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse> {
        debug!(username = %username, "Attempting login");

        // Generate security headers
        let security_headers = self.client.generate_security_headers();

        // Build request body
        let mut form_data = HashMap::new();
        form_data.insert("client_id", "MOBrBDS8blbauoSck0ZfDbtuzpyT");
        form_data.insert("client_secret", "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj");
        form_data.insert("grant_type", "password");
        form_data.insert("username", username);
        form_data.insert("password", password);
        form_data.insert("get_secure_url", "true");

        // Send authentication request
        let mut request = self.client.client.post(&self.auth_url);
        
        // Add security headers
        for (key, value) in security_headers {
            request = request.header(&key, value);
        }

        // Add form data
        request = request.form(&form_data);

        let response = request.send().await?;

        // Check response status
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Failed to get error information".to_string());
            warn!(error = %error_text, "Login failed");
            return Err(PixivError::AuthError(format!("Login failed: {}", error_text)));
        }

        // Parse response
        let mut auth_response: AuthResponse = response.json().await?;
        auth_response.obtained_at = Utc::now();

        // Update client tokens
        self.client.set_access_token(auth_response.access_token.clone());
        self.client.set_refresh_token(auth_response.refresh_token.clone());

        info!(user_id = %auth_response.user.id, "Login successful");
        Ok(auth_response)
    }

    /// Get new access token using refresh token
    pub async fn refresh_access_token(&mut self, refresh_token: &str) -> Result<AuthResponse> {
        debug!("Refreshing access token");

        // Generate security headers
        let security_headers = self.client.generate_security_headers();

        // Build request body
        let mut form_data = HashMap::new();
        form_data.insert("client_id", "MOBrBDS8blbauoSck0ZfDbtuzpyT");
        form_data.insert("client_secret", "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj");
        form_data.insert("grant_type", "refresh_token");
        form_data.insert("refresh_token", refresh_token);
        form_data.insert("get_secure_url", "true");

        // Send refresh request
        let mut request = self.client.client.post(&self.auth_url);
        
        // Add security headers
        for (key, value) in security_headers {
            request = request.header(&key, value);
        }

        // Add form data
        request = request.form(&form_data);

        let response = request.send().await?;

        // Check response status
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "Failed to get error information".to_string());
            warn!(error = %error_text, "Token refresh failed");
            return Err(PixivError::AuthError(format!("Token refresh failed: {}", error_text)));
        }

        // Parse response
        let mut auth_response: AuthResponse = response.json().await?;
        auth_response.obtained_at = Utc::now();

        // Update client tokens
        self.client.set_access_token(auth_response.access_token.clone());
        self.client.set_refresh_token(auth_response.refresh_token.clone());

        info!("Access token refreshed successfully");
        Ok(auth_response)
    }

    /// Check if access token is expired
    pub fn is_token_expired(&self, auth_response: &AuthResponse) -> bool {
        let now = Utc::now();
        let expires_at = auth_response.obtained_at + chrono::Duration::seconds(auth_response.expires_in as i64);
        
        // Consider token expired 5 minutes in advance
        let buffer = chrono::Duration::minutes(5);
        now + buffer > expires_at
    }

    /// Get mutable reference to HTTP client
    pub fn client_mut(&mut self) -> &mut HttpClient {
        &mut self.client
    }

    /// Get immutable reference to HTTP client
    pub fn client(&self) -> &HttpClient {
        &self.client
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_client_creation() {
        let result = AuthClient::new();
        assert!(result.is_ok());
    }

    #[test]
    fn test_token_expiry_check() {
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
}