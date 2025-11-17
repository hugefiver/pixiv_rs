use thiserror::Error;
use std::net::AddrParseError;
use std::collections::HashMap;

/// API error codes
#[derive(Error, Debug, Clone)]
pub enum ApiErrorCode {
    /// Authentication error (103)
    #[error("Authentication error (103)")]
    AuthError103,
    
    /// Server error (500)
    #[error("Server error (500)")]
    ServerError500,
    
    /// Too many requests (429)
    #[error("Too many requests (429)")]
    TooManyRequests429,
    
    /// Not found (404)
    #[error("Not found (404)")]
    NotFound404,
    
    /// Forbidden (403)
    #[error("Forbidden (403)")]
    Forbidden403,
    
    /// Bad request (400)
    #[error("Bad request (400)")]
    BadRequest400,
    
    /// Service unavailable (503)
    #[error("Service unavailable (503)")]
    ServiceUnavailable503,
    
    /// Unknown error code
    #[error("Unknown error code: {0}")]
    Unknown(String),
}

impl From<&str> for ApiErrorCode {
    fn from(code: &str) -> Self {
        match code {
            "103" => ApiErrorCode::AuthError103,
            "500" => ApiErrorCode::ServerError500,
            "429" => ApiErrorCode::TooManyRequests429,
            "404" => ApiErrorCode::NotFound404,
            "403" => ApiErrorCode::Forbidden403,
            "400" => ApiErrorCode::BadRequest400,
            "503" => ApiErrorCode::ServiceUnavailable503,
            _ => ApiErrorCode::Unknown(code.to_string()),
        }
    }
}

/// API error details
#[derive(Debug, Clone)]
pub struct ApiErrorDetails {
    /// Error code
    pub code: ApiErrorCode,
    /// Error message
    pub message: String,
    /// Response headers
    pub headers: Option<HashMap<String, String>>,
    /// Response body
    pub body: Option<String>,
}

/// Pixiv API error types
#[derive(Error, Debug)]
pub enum PixivError {
    /// Network related errors
    #[error("Network error: {0}")]
    NetworkError(#[from] NetworkError),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    AuthError(String),

    /// API returned error
    #[error("API error: {0}")]
    ApiError(String),
    
    /// API returned error (with details)
    #[error("API error: {details:?}")]
    ApiErrorWithDetails {
        /// Error details
        details: ApiErrorDetails,
    },

    /// JSON parsing error
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Illustration not found
    #[error("Illustration not found: {0}")]
    IllustNotFound(u64),

    /// Illustration access restricted
    #[error("Illustration access restricted: {0}")]
    IllustRestricted(u64),

    /// Search parameter error
    #[error("Search parameter error: {0}")]
    SearchParameterError(String),

    /// Ranking parameter error
    #[error("Ranking parameter error: {0}")]
    RankingParameterError(String),

    /// Recommended parameter error
    #[error("Recommended parameter error: {0}")]
    RecommendedParameterError(String),

    /// Unknown error
    #[error("Unknown error: {0}")]
    Unknown(String),
    
    /// Public API error
    #[error("Public API error: {0}")]
    PublicApiError(String),
    
    /// SNI bypass error
    #[error("SNI bypass error: {0}")]
    SniBypassError(#[from] SniBypassError),
}

/// Network related errors
#[derive(Error, Debug)]
pub enum NetworkError {
    /// HTTP request error
    #[error("HTTP request error: {0}")]
    RequestError(#[from] reqwest::Error),

    /// Connection timeout
    #[error("Connection timeout")]
    Timeout,

    /// Invalid URL
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

/// SNI bypass related errors
#[derive(Error, Debug)]
pub enum SniBypassError {
    /// Invalid IP address
    #[error("Invalid IP address: {0}")]
    InvalidIp(String),
    
    /// Connection error
    #[error("Connection error: {0}")]
    ConnectionError(#[from] reqwest::Error),
    
    /// IP parsing error
    #[error("IP parsing error: {0}")]
    IpParseError(#[from] AddrParseError),
}

impl From<reqwest::Error> for PixivError {
    fn from(err: reqwest::Error) -> Self {
        PixivError::NetworkError(NetworkError::RequestError(err))
    }
}

/// Pixiv API result type
pub type Result<T> = std::result::Result<T, PixivError>;