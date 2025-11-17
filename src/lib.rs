//! Pixiv API Rust Client
//!
//! This is a Rust client library for interacting with Pixiv API, providing authentication, network requests and other features.
//!
//! ## Features
//!
//! - OAuth2 password credential authentication
//! - Automatic token refresh
//! - Support for multiple runtimes (tokio/async-std/smol)
//! - Comprehensive error handling
//! - Detailed logging
//! - SNI bypass support (for bypassing network restrictions)
//!
//! ## Quick Start
//!
//! ```rust
//! use pixiv_rs::auth::AuthClient;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize logging
//!     tracing_subscriber::fmt::init();
//!
//!     // Create authentication client
//!     let mut auth_client = AuthClient::new()?;
//!
//!     // Login with username and password
//!     let auth_response = auth_client.login("your_username", "your_password").await?;
//!
//!     println!("Login successful! User ID: {}", auth_response.user.id);
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

// Re-export common types and functions
pub use auth::{AuthClient, AuthResponse, User as AuthUser};
pub use client::app::AppClient;
pub use client::public::PublicClient;
pub use client::bypass_sni::BypassSniAppClient;

/// Example of using SNI bypass feature
///
/// ```rust
/// use pixiv_rs::client::bypass_sni::BypassSniAppClient;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Use IP address to bypass SNI restrictions
///     let client = BypassSniAppClient::with_ip("210.140.131.145")?;
///
///     // Set access token
///     client.http_client.set_access_token("your_access_token".to_string());
///
///     // Call API
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

/// Library version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Library name
pub const NAME: &str = env!("CARGO_PKG_NAME");

/// Initialize logger
///
/// This is a convenience function for initializing the default logger.
/// In production environments, you may need to configure more complex logging settings.
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
        // If tracing-subscriber is not enabled, do not initialize logger
        // Users need to configure the logger themselves
        eprintln!("Warning: tracing-subscriber feature not enabled, logs may not be displayed");
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