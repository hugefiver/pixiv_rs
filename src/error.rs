use thiserror::Error;
use std::net::AddrParseError;
use std::collections::HashMap;

/// API错误代码
#[derive(Error, Debug, Clone)]
pub enum ApiErrorCode {
    /// 认证错误 (103)
    #[error("认证错误 (103)")]
    AuthError103,
    
    /// 服务器错误 (500)
    #[error("服务器错误 (500)")]
    ServerError500,
    
    /// 请求过多 (429)
    #[error("请求过多 (429)")]
    TooManyRequests429,
    
    /// 未找到 (404)
    #[error("未找到 (404)")]
    NotFound404,
    
    /// 禁止访问 (403)
    #[error("禁止访问 (403)")]
    Forbidden403,
    
    /// 错误请求 (400)
    #[error("错误请求 (400)")]
    BadRequest400,
    
    /// 服务不可用 (503)
    #[error("服务不可用 (503)")]
    ServiceUnavailable503,
    
    /// 未知错误代码
    #[error("未知错误代码: {0}")]
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

/// API错误详情
#[derive(Debug, Clone)]
pub struct ApiErrorDetails {
    /// 错误代码
    pub code: ApiErrorCode,
    /// 错误消息
    pub message: String,
    /// 响应头
    pub headers: Option<HashMap<String, String>>,
    /// 响应体
    pub body: Option<String>,
}

/// Pixiv API 错误类型
#[derive(Error, Debug)]
pub enum PixivError {
    /// 网络相关错误
    #[error("网络错误: {0}")]
    NetworkError(#[from] NetworkError),

    /// 认证错误
    #[error("认证错误: {0}")]
    AuthError(String),

    /// API 返回错误
    #[error("API 错误: {0}")]
    ApiError(String),
    
    /// API 返回错误（带详细信息）
    #[error("API 错误: {details:?}")]
    ApiErrorWithDetails {
        /// 错误详情
        details: ApiErrorDetails,
    },

    /// JSON 解析错误
    #[error("JSON 解析错误: {0}")]
    JsonError(#[from] serde_json::Error),

    /// 插画不存在
    #[error("插画不存在: {0}")]
    IllustNotFound(u64),

    /// 插画访问受限
    #[error("插画访问受限: {0}")]
    IllustRestricted(u64),

    /// 搜索参数错误
    #[error("搜索参数错误: {0}")]
    SearchParameterError(String),

    /// 排行榜参数错误
    #[error("排行榜参数错误: {0}")]
    RankingParameterError(String),

    /// 推荐参数错误
    #[error("推荐参数错误: {0}")]
    RecommendedParameterError(String),

    /// 未知错误
    #[error("未知错误: {0}")]
    Unknown(String),
    
    /// Public API 错误
    #[error("Public API 错误: {0}")]
    PublicApiError(String),
    
    /// SNI绕过错误
    #[error("SNI绕过错误: {0}")]
    SniBypassError(#[from] SniBypassError),
}

/// 网络相关错误
#[derive(Error, Debug)]
pub enum NetworkError {
    /// HTTP 请求错误
    #[error("HTTP 请求错误: {0}")]
    RequestError(#[from] reqwest::Error),

    /// 连接超时
    #[error("连接超时")]
    Timeout,

    /// 无效的 URL
    #[error("无效的 URL: {0}")]
    InvalidUrl(String),
}

/// SNI绕过相关错误
#[derive(Error, Debug)]
pub enum SniBypassError {
    /// 无效的IP地址
    #[error("无效的IP地址: {0}")]
    InvalidIp(String),
    
    /// 连接错误
    #[error("连接错误: {0}")]
    ConnectionError(#[from] reqwest::Error),
    
    /// IP解析错误
    #[error("IP解析错误: {0}")]
    IpParseError(#[from] AddrParseError),
}

impl From<reqwest::Error> for PixivError {
    fn from(err: reqwest::Error) -> Self {
        PixivError::NetworkError(NetworkError::RequestError(err))
    }
}

/// Pixiv API 结果类型
pub type Result<T> = std::result::Result<T, PixivError>;