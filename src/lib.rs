//! Pixiv API Rust客户端
//!
//! 这是一个用于与Pixiv API交互的Rust客户端库，提供了认证、网络请求等功能。
//!
//! ## 功能特性
//!
//! - OAuth2密码凭证认证
//! - 自动令牌刷新
//! - 支持多运行时（tokio/async-std/smol）
//! - 完善的错误处理
//! - 详细的日志记录
//! - SNI绕过支持（用于绕过网络限制）
//!
//! ## 快速开始
//!
//! ```rust
//! use pixiv_rs::auth::AuthClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // 初始化日志
//!     tracing_subscriber::fmt::init();
//!
//!     // 创建认证客户端
//!     let mut auth_client = AuthClient::new()?;
//!
//!     // 使用用户名和密码登录
//!     let auth_response = auth_client.login("your_username", "your_password").await?;
//!
//!     println!("登录成功！用户ID: {}", auth_response.user.id);
//!
//!     Ok(())
//! }
//! ```

pub mod auth;
pub mod client {
    pub mod app;
    pub mod public;
    pub mod bypass_sni;
}
pub mod error;
pub mod models {
    pub mod app;
    pub mod public;
}
pub mod network;
pub mod utils;

// 重新导出常用类型和函数
pub use auth::{AuthClient, AuthResponse, User as AuthUser};
pub use client::app::AppClient;
pub use client::public::PublicClient;
pub use client::bypass_sni::BypassSniAppClient;

/// 使用SNI绕过功能的示例
///
/// ```rust
/// use pixiv_rs::client::bypass_sni::BypassSniAppClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // 使用IP地址绕过SNI限制
///     let client = BypassSniAppClient::with_ip("210.140.131.145")?;
///
///     // 设置访问令牌
///     client.http_client.set_access_token("your_access_token".to_string());
///
///     // 调用API
///     // let illust = client.illust_detail(12345).await?;
///
///     Ok(())
/// }
/// ```
pub use error::{ApiErrorCode, ApiErrorDetails, PixivError, Result};
pub use models::app::{
    Comment, CommentAccessControl, CommentsResponse, ContentType, Duration, Filter, FollowRestrict,
    Illust, IllustBookmarkResponse, IllustDetail, IllustFollowResponse, ImageUrls, MetaPage, MetaSinglePage,
    ProfileImageUrls, RankingLabel, RankingMode, RankingResponse, RecommendedResponse, RestrictionAttributes,
    SearchIllustResponse, SearchTarget, Series, Sort, Tag, TrendingTag, TrendingTagsResponse,
    UgoiraFrame, UgoiraMetadata, UgoiraMetadataResponse, User as AppUser, UserFollowerResponse,
    UserFollowingResponse, UserMypixivResponse, UserPreview, ZipUrls,
};
pub use models::public::{
    PublicIllust, PublicUser, PublicSearchResponse, PublicUserDetail, PublicUserIllusts, PublicUserBookmarks,
    PublicSearchResponse as PublicSearchIllustResponse, SearchTarget as PublicSearchTarget, Sort as PublicSort,
    Restrict as PublicRestrict, ContentType as PublicContentType, Duration as PublicDuration, Filter as PublicFilter
};
pub use network::HttpClient;
pub use utils::{download, extract_extension, format_file_size, parse_qs, safe_filename, set_accept_language};

/// 库版本信息
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// 库名称
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// 初始化日志记录器
/// 
/// 这是一个便捷函数，用于初始化默认的日志记录器。
/// 在生产环境中，你可能需要配置更复杂的日志设置。
pub fn init_logging() {
    #[cfg(feature = "tracing-subscriber")]
    {
        use tracing_subscriber::{fmt, EnvFilter};
        
        fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }
    
    #[cfg(not(feature = "tracing-subscriber"))]
    {
        // 如果没有tracing-subscriber，则不初始化日志记录器
        // 用户需要自己配置日志记录器
        eprintln!("警告: 未启用tracing-subscriber功能，日志可能不会显示");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_info() {
        assert!(!VERSION.is_empty());
        assert_eq!(NAME, "pixiv_rs");
    }
}