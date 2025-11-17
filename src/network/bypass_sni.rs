use crate::error::{NetworkError, PixivError, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;
use std::net::IpAddr;
use tracing::debug;

/// SNI bypass HTTP client for accessing Pixiv API by bypassing network restrictions
///
/// This client bypasses SNI (Server Name Indication) restrictions by specifying IP address,
/// allowing access to Pixiv API in certain network environments.
///
/// # Example
///
/// ```rust
/// use pixiv_rs::network::bypass_sni::BypassSniClient;
///
/// // Create client using Pixiv API IP address
/// let client = BypassSniClient::new("210.140.131.145");
/// ```
#[derive(Debug, Clone)]
pub struct BypassSniClient {
    /// Internal reqwest client
    pub(crate) client: Client,
    /// Authentication token
    access_token: Option<String>,
    /// Refresh token
    refresh_token: Option<String>,
    /// API base URL
    base_url: String,
    /// IP address for bypass
    pub ip: IpAddr,
}

impl BypassSniClient {
    /// Create new SNI bypass HTTP client instance
    pub fn new(ip: &str) -> Result<Self> {
        let ip = ip
            .parse::<std::net::IpAddr>()
            .map_err(|_| PixivError::NetworkError(NetworkError::InvalidUrl(format!(
                "Invalid IP address: {}",
                ip
            ))))?;

        tracing::info!(ip = %ip, "Using SNI bypass with IP address");

        // Create SNI bypass client
        let mut builder = reqwest::Client::builder();
        // Use port 443 for SNI bypass
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

    /// Set authentication token
    pub fn set_access_token(&mut self, token: String) {
        self.access_token = Some(token);
    }

    /// Get current authentication token
    pub fn access_token(&self) -> Option<&str> {
        self.access_token.as_deref()
    }

    /// Set refresh token
    pub fn set_refresh_token(&mut self, token: String) {
        self.refresh_token = Some(token);
    }

    /// Get current refresh token
    pub fn refresh_token(&self) -> Option<&str> {
        self.refresh_token.as_deref()
    }

    /// Send GET request
    pub async fn get(&self, url: &str) -> Result<Response> {
        self.send_request(reqwest::Method::GET, url, None::<&()>).await
    }

    /// Send POST request
    pub async fn post<T: Serialize + ?Sized>(&self, url: &str, body: &T) -> Result<Response> {
        self.send_request(reqwest::Method::POST, url, Some(body)).await
    }

    /// Send authenticated API request
    pub async fn send_request<T: Serialize + ?Sized>(
        &self,
        method: reqwest::Method,
        url: &str,
        body: Option<&T>,
    ) -> Result<Response> {
        debug!(method = %method, url = %url, "Sending API request with SNI bypass");

        // Set Host header
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::HOST,
            reqwest::header::HeaderValue::from_static("app-api.pixiv.net"),
        );

        let mut request = self.client.request(method.clone(), url).headers(headers);

        // Add authentication header
        if let Some(token) = &self.access_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        // Add request body
        if let Some(body) = body {
            request = request.json(body);
        }

        // Send request
        let response = request.send().await?;

        // Check response status
        if !response.status().is_success() {
            return Err(PixivError::ApiError(format!(
                "API request failed: {} - {}",
                response.status(),
                response.text().await.unwrap_or_else(|_| "Failed to get error information".to_string())
            )));
        }

        debug!(status = %response.status(), "API request completed successfully");
        Ok(response)
    }

    /// Get API base URL
    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    /// Set API base URL
    pub fn set_base_url(&mut self, base_url: String) {
        self.base_url = base_url;
    }

    /// Generate security headers
    pub fn generate_security_headers(&self) -> HashMap<String, String> {
        use chrono::Utc;
        use md5::compute;

        let mut headers = HashMap::new();

        // Generate x-client-time and x-client-hash
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