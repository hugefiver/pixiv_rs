use crate::error::{NetworkError, PixivError, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use tracing::debug;

/// SNI绕过HTTP客户端，用于绕过网络限制访问Pixiv API
///
/// 这个客户端通过指定IP地址来绕过SNI（Server Name Indication）限制，
/// 从而在某些网络环境下访问Pixiv API。
///
/// # 示例
///
/// ```rust
/// use pixiv_rs::network::bypass_sni::BypassSniClient;
///
/// // 使用Pixiv API的IP地址创建客户端
/// let client = BypassSniClient::new("210.140.131.145");
/// ```
#[derive(Debug, Clone)]
pub struct BypassSniClient {
    /// 内部reqwest客户端
    pub(crate) client: Client,
    /// 认证令牌
    access_token: Option<String>,
    /// 刷新令牌
    refresh_token: Option<String>,
    /// API基础URL
    base_url: String,
    /// 用于绕过的IP地址
    pub ip: IpAddr,
}

impl BypassSniClient {
    /// 创建新的SNI绕过HTTP客户端实例
    pub fn new(ip: &str) -> Result<Self> {
        let ip = ip
            .parse::<std::net::IpAddr>()
            .map_err(|_| PixivError::NetworkError(NetworkError::InvalidUrl(format!(
                "Invalid IP address: {}",
                ip
            ))))?;

        tracing::info!(ip = %ip, "Using SNI bypass with IP address");

        // 创建绕过SNI的客户端
        let mut builder = reqwest::Client::builder();
        // 使用端口443进行SNI绕过
        let socket_addr = std::net::SocketAddr::new(ip, 443);
        builder = builder
            .danger_accept_invalid_certs(true)
            .resolve("app-api.pixiv.net", socket_addr);

        let client = builder
            .build()
            .map_err(|e| PixivError::NetworkError(NetworkError::RequestError(e)))?;

        Ok(Self {
            client,
            access_token: None,
            refresh_token: None,
            base_url: format!("https://{}", ip),
            ip,
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
        debug!(method = %method, url = %url, "Sending API request with SNI bypass");

        // 设置Host头
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static("app-api.pixiv.net"),
        );

        let mut request = self.client.request(method.clone(), url).headers(headers);

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
    fn test_bypass_sni_client_creation() {
        let result = BypassSniClient::new("210.140.131.145");
        assert!(result.is_ok());
    }

    #[test]
    fn test_invalid_ip() {
        let result = BypassSniClient::new("invalid_ip");
        assert!(result.is_err());
    }
}