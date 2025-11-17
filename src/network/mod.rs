pub mod bypass_sni;

use crate::error::{NetworkError, PixivError, Result};
use reqwest::{Client, Response};
use serde::Serialize;
use std::collections::HashMap;
use tracing::debug;

/// HTTP client for communicating with Pixiv API
#[derive(Debug, Clone)]
pub struct HttpClient {
    /// Internal reqwest client
    pub client: Client,
    /// Authentication token
    access_token: Option<String>,
    /// Refresh token
    refresh_token: Option<String>,
    /// API base URL
    base_url: String,
}

impl HttpClient {
    /// Create new HTTP client instance
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
        debug!(method = %method, url = %url, "Sending API request");

        let mut request = self.client.request(method.clone(), url);

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
    fn test_generate_security_headers() {
        let client = HttpClient::new().unwrap();
        let headers = client.generate_security_headers();
        
        assert!(headers.contains_key("x-client-time"));
        assert!(headers.contains_key("x-client-hash"));
        assert!(!headers.get("x-client-time").unwrap().is_empty());
        assert!(!headers.get("x-client-hash").unwrap().is_empty());
    }
}