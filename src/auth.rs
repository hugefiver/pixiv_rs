use crate::error::{PixivError, Result};
use crate::network::HttpClient;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info, warn};

/// Pixiv API 认证客户端
#[derive(Debug, Clone)]
pub struct AuthClient {
    /// HTTP客户端
    client: HttpClient,
    /// 认证基础URL
    auth_url: String,
}

/// 认证响应数据
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    /// 访问令牌
    pub access_token: String,
    /// 刷新令牌
    pub refresh_token: String,
    /// 令牌类型
    pub token_type: String,
    /// 过期时间（秒）
    pub expires_in: u64,
    /// 用户信息
    pub user: User,
    /// 令牌获取时间
    #[serde(skip)]
    pub obtained_at: DateTime<Utc>,
}

/// 用户信息
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// 用户ID
    pub id: u64,
    /// 用户名
    pub name: String,
    /// 账户名
    pub account: String,
    /// 邮箱
    pub email: Option<String>,
    /// 头像URL
    pub profile_image_urls: ProfileImageUrls,
}

/// 用户头像URL
#[derive(Debug, Serialize, Deserialize)]
pub struct ProfileImageUrls {
    /// 小尺寸头像
    pub px_16x16: Option<String>,
    /// 中尺寸头像
    pub px_50x50: Option<String>,
    /// 大尺寸头像
    pub px_170x170: Option<String>,
}

impl AuthClient {
    /// 创建新的认证客户端
    pub fn new() -> Result<Self> {
        let client = HttpClient::new()?;
        Ok(Self {
            client,
            auth_url: "https://oauth.secure.pixiv.net/auth/token".to_string(),
        })
    }

    /// 使用用户名和密码登录
    pub async fn login(&mut self, username: &str, password: &str) -> Result<AuthResponse> {
        debug!(username = %username, "Attempting login");

        // 生成安全校验头
        let security_headers = self.client.generate_security_headers();

        // 构建请求体
        let mut form_data = HashMap::new();
        form_data.insert("client_id", "MOBrBDS8blbauoSck0ZfDbtuzpyT");
        form_data.insert("client_secret", "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj");
        form_data.insert("grant_type", "password");
        form_data.insert("username", username);
        form_data.insert("password", password);
        form_data.insert("get_secure_url", "true");

        // 发送认证请求
        let mut request = self.client.client.post(&self.auth_url);
        
        // 添加安全校验头
        for (key, value) in security_headers {
            request = request.header(&key, value);
        }

        // 添加表单数据
        request = request.form(&form_data);

        let response = request.send().await?;

        // 检查响应状态
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "无法获取错误信息".to_string());
            warn!(error = %error_text, "Login failed");
            return Err(PixivError::AuthError(format!("登录失败: {}", error_text)));
        }

        // 解析响应
        let mut auth_response: AuthResponse = response.json().await?;
        auth_response.obtained_at = Utc::now();

        // 更新客户端的令牌
        self.client.set_access_token(auth_response.access_token.clone());
        self.client.set_refresh_token(auth_response.refresh_token.clone());

        info!(user_id = %auth_response.user.id, "Login successful");
        Ok(auth_response)
    }

    /// 使用刷新令牌获取新的访问令牌
    pub async fn refresh_access_token(&mut self, refresh_token: &str) -> Result<AuthResponse> {
        debug!("Refreshing access token");

        // 生成安全校验头
        let security_headers = self.client.generate_security_headers();

        // 构建请求体
        let mut form_data = HashMap::new();
        form_data.insert("client_id", "MOBrBDS8blbauoSck0ZfDbtuzpyT");
        form_data.insert("client_secret", "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj");
        form_data.insert("grant_type", "refresh_token");
        form_data.insert("refresh_token", refresh_token);
        form_data.insert("get_secure_url", "true");

        // 发送刷新请求
        let mut request = self.client.client.post(&self.auth_url);
        
        // 添加安全校验头
        for (key, value) in security_headers {
            request = request.header(&key, value);
        }

        // 添加表单数据
        request = request.form(&form_data);

        let response = request.send().await?;

        // 检查响应状态
        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_else(|_| "无法获取错误信息".to_string());
            warn!(error = %error_text, "Token refresh failed");
            return Err(PixivError::AuthError(format!("令牌刷新失败: {}", error_text)));
        }

        // 解析响应
        let mut auth_response: AuthResponse = response.json().await?;
        auth_response.obtained_at = Utc::now();

        // 更新客户端的令牌
        self.client.set_access_token(auth_response.access_token.clone());
        self.client.set_refresh_token(auth_response.refresh_token.clone());

        info!("Access token refreshed successfully");
        Ok(auth_response)
    }

    /// 检查访问令牌是否过期
    pub fn is_token_expired(&self, auth_response: &AuthResponse) -> bool {
        let now = Utc::now();
        let expires_at = auth_response.obtained_at + chrono::Duration::seconds(auth_response.expires_in as i64);
        
        // 提前5分钟认为令牌过期
        let buffer = chrono::Duration::minutes(5);
        now + buffer > expires_at
    }

    /// 获取HTTP客户端的可变引用
    pub fn client_mut(&mut self) -> &mut HttpClient {
        &mut self.client
    }

    /// 获取HTTP客户端的不可变引用
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