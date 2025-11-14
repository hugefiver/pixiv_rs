pub mod bypass_sni;

use crate::error::{NetworkError, PixivError, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;
use tracing::debug;

/// HTTP客户端，用于与Pixiv API通信
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// 内部reqwest客户端
    pub client: Client,
    /// 认证令牌
    access_token: Option<String>,
    /// 刷新令牌
    refresh_token: Option<String>,
    /// API基础URL
    base_url: String,
}

impl HttpClient {
    /// 创建新的HTTP客户端实例
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .user_agent("PixivRustClient/0.1.0")
            .build()
            .map_err(NetworkError::RequestError)?;

        Ok(Self {
            client,
            access_token: None,
            refresh_token: None,
            base_url: "https://app-api.pixiv.net".to_string(),
        })
    }

    /// 设置认证令牌
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }

    /// 获取当前认证令牌
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    /// 设置刷新令牌
    pub fn set_refresh_token(&mut self, token: String) {
        self.refresh_token = Some(token);
    }

    /// 获取当前刷新令牌
    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }

    /// 发送GET请求
    pub async fn get(&self, url: &str) -> Result<Response> {
        self.send_request(reqwest::Method::GET, url, None::<&()>).await
    }

    /// 发送POST请求
    pub async fn post<T: Serialize + ?Sized>(&self, url: &str, body: &T) -> Result<Response> {
        self.send_request(reqwest::Method::POST, url, Some(body)).await
    }

    /// 发送带认证的API请求
    pub async fn send_request<T: Serialize + ?Sized>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&T>,
    ) -> Result<Response> {
        debug!(method = %method, url = %url, "Sending API request");

        let mut request = self.client.request(method.clone(), url);

        // 添加认证头
        if let Some(token) = &self.access_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        // 添加请求体
        if let Some(body) = body {
            request = request.json(body);
        }

        // 发送请求
        let response = request.send().await?;

        // 检查响应状态
        if !response.status().is_success() {
            return Err(PixivError::ApiError(format!(
                "API请求失败: {} - {}",
                response.status(),
                response.text().await.unwrap_or_else(|_| "无法获取错误信息".to_string())
            )));
        }

        debug!(status = %response.status(), "API request completed successfully");
        Ok(response)
    }

    /// 获取API基础URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// 设置API基础URL
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    /// 生成安全校验头
    pub fn generate_security_headers(&self) -> HashMap<String, String> {
        use chrono::Utc;
        use md5::compute;

        let mut headers = HashMap::new();
        
        // 生成x-client-time和x-client-hash
        let local_time = Utc::now().format("%Y-%m-%dT%H:%M:%S+00:00").to_string();
        let hash_input = format!("{}{}", local_time, "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c");
        let hash = format!("{:x}", compute(hash_input));

        headers.insert("x-client-time".to_string(), local_time);
        headers.insert("x-client-hash".to_string(), hash);

        headers
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_security_headers() {
        let client = HttpClient::new().unwrap();
        let headers = client.generate_security_headers();
        
        assert!(headers.contains_key("x-client-time"));
        assert!(headers.contains_key("x-client-hash"));
        assert!(!headers.get("x-client-time").unwrap().is_empty());
        assert!(!headers.get("x-client-hash").unwrap().is_empty());
    }
}