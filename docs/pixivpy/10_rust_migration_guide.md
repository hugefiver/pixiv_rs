# Rust Migration Guide

## Overview

This guide provides a comprehensive roadmap for porting PixivPy from Python to Rust. It covers architecture decisions, implementation patterns, and best practices for creating a functionally equivalent Rust library.

## Architecture Mapping

### Python to Rust Module Mapping

```
PixivPy Structure → Rust Structure
├── pixivpy/          → crates/
│   ├── api.py        → pixiv-rs-core/
│   ├── aapi.py       → pixiv-rs-client/
│   ├── bapi.py       → pixiv-rs-bypass/
│   ├── models/       → pixiv-rs-models/
│   ├── exceptions.py  → pixiv-rs-error/
│   └── utils/        → pixiv-rs-utils/
```

### Workspace Structure
```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "pixiv-rs-core",
    "pixiv-rs-client",
    "pixiv-rs-bypass",
    "pixiv-rs-models",
    "pixiv-rs-error",
    "pixiv-rs-utils",
]
```

## Core Implementation Strategy

### 1. Error Type System

**Python Pattern**:
```python
class PixivError(Exception):
    def __init__(self, reason, header=None, body=None):
        self.reason = reason
        self.header = header
        self.body = body
```

**Rust Implementation**:
```rust
// pixiv-rs-error/src/lib.rs
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PixivError {
    #[error("Authentication failed: {message}")]
    AuthError {
        message: String,
        #[from]
        source: reqwest::Error,
    },

    #[error("Rate limit exceeded: retry after {retry_after:?}")]
    RateLimitError {
        retry_after: Option<Duration>,
    },

    #[error("Network error: {message}")]
    NetworkError {
        message: String,
        #[source]
        source: Option<reqwest::Error>,
    },

    #[error("Validation error: {field} - {message}")]
    ValidationError {
        field: String,
        message: String,
    },
}

// Result type alias
pub type PixivResult<T> = Result<T, PixivError>;
```

### 2. Data Models

**Python Pattern**:
```python
from pydantic import BaseModel
from typing import Optional

class IllustInfo(BaseModel):
    id: int
    title: str
    user: UserInfo
    create_date: datetime

    class Config:
        allow_population_by_field_name = True
```

**Rust Implementation**:
```rust
// pixiv-rs-models/src/illust.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use super::user::UserInfo;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IllustInfo {
    pub id: u64,
    pub title: String,
    pub user: UserInfo,
    #[serde(with = "crate::models::datetime_format")]
    pub create_date: DateTime<Utc>,
    // ... other fields
}

// Custom datetime format module
pub mod datetime_format {
    use chrono::{DateTime, Utc, TimeZone};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%:z";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}
```

### 3. HTTP Client

**Python Pattern**:
```python
class BasePixivAPI:
    def __init__(self, **kwargs):
        self.session = requests.Session()
        self.session.headers.update({"User-Agent": self.USER_AGENT})

    def get(self, url, params=None):
        return self.session.get(url, params=params)
```

**Rust Implementation**:
```rust
// pixiv-rs-core/src/client.rs
use reqwest::{Client, Response, RequestBuilder};
use std::sync::Arc;

#[derive(Clone)]
pub struct PixivClient {
    client: Client,
    config: Arc<ClientConfig>,
}

impl PixivClient {
    pub fn new(config: ClientConfig) -> PixivResult<Self> {
        let client = Client::builder()
            .user_agent(&config.user_agent)
            .default_headers(self.build_headers(&config)?)
            .build()?;

        Ok(Self {
            client,
            config: Arc::new(config),
        })
    }

    pub async fn get(&self, url: &str, params: Option<&[(&str, &str)]>) -> PixivResult<Response> {
        let mut request = self.client.get(url);

        if let Some(params) = params {
            request = request.query(params);
        }

        let response = request.send().await?;
        self.check_response(response).await
    }

    async fn check_response(&self, response: Response) -> PixivResult<Response> {
        let status = response.status();

        if status.is_success() {
            Ok(response)
        } else {
            Err(match status.as_u16() {
                401 => PixivError::AuthError {
                    message: "Unauthorized".to_string(),
                    source: reqwest::Error::from(response.error_for_status().unwrap_err()),
                },
                429 => PixivError::RateLimitError {
                    retry_after: response.headers()
                        .get("Retry-After")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse().ok())
                        .map(Duration::from_secs),
                },
                _ => PixivError::NetworkError {
                    message: format!("HTTP {}", status),
                    source: None,
                },
            })
        }
    }
}
```

## Authentication Implementation

### 1. OAuth2 Flow

**Python Pattern**:
```python
def auth(self, refresh_token):
    headers = self.generate_client_headers()
    data = {
        "client_id": self.client_id,
        "refresh_token": refresh_token,
        "grant_type": "refresh_token",
    }
    response = self.session.post(self.AUTH_URL, data=data, headers=headers)
    return response.json()
```

**Rust Implementation**:
```rust
// pixiv-rs-core/src/auth.rs
use chrono::Utc;
use sha2::{Digest, Sha256};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Debug)]
pub struct AuthManager {
    client: Client,
    client_id: String,
    client_secret: String,
    hash_secret: String,
}

impl AuthManager {
    pub async fn auth(&self, refresh_token: &str) -> PixivResult<AuthResponse> {
        let timestamp = Utc::now().timestamp().to_string();
        let hash_input = format!("{}{}", timestamp, self.hash_secret);
        let client_hash = format!("{:x}", Sha256::digest(hash_input.as_bytes()));

        let mut headers = HeaderMap::new();
        headers.insert("X-Client-Time", timestamp.parse()?);
        headers.insert("X-Client-Hash", client_hash.parse()?);
        headers.insert("Content-Type", "application/x-www-form-urlencoded".parse()?);

        let form_data = [
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ];

        let response = self.client
            .post("https://oauth.secure.pixiv.net/auth/token")
            .headers(headers)
            .form(&form_data)
            .send()
            .await?;

        let auth_response: AuthResponse = response.json().await?;
        Ok(auth_response)
    }
}
```

## API Endpoint Implementation

### 1. Trait-based Architecture

**Python Pattern**:
```python
class AppPixivAPI(BasePixivAPI):
    def illust_detail(self, illust_id: int):
        return self.get(f"/v1/illust/detail", params={"illust_id": illust_id})
```

**Rust Implementation**:
```rust
// pixiv-rs-client/src/illust.rs
use async_trait::async_trait;
use pixiv_rs_core::client::PixivClient;
use pixiv_rs_models::illust::IllustInfo;
use pixiv_rs_error::PixivResult;

#[async_trait]
pub trait IllustAPI {
    async fn illust_detail(&self, illust_id: u64) -> PixivResult<IllustInfo>;
    async fn illust_ranking(&self, mode: RankingMode) -> PixivResult<RankingInfo>;
    // ... other methods
}

pub struct PixivIllustClient {
    client: PixivClient,
}

#[async_trait]
impl IllustAPI for PixivIllustClient {
    async fn illust_detail(&self, illust_id: u64) -> PixivResult<IllustInfo> {
        let response = self
            .client
            .get(&format!("/v1/illust/detail"))
            .query(&[("illust_id", &illust_id.to_string())])
            .await?;

        let result: IllustDetailResponse = response.json().await?;
        Ok(result.illust)
    }

    async fn illust_ranking(&self, mode: RankingMode) -> PixivResult<RankingInfo> {
        let response = self
            .client
            .get("/v1/illust/ranking")
            .query(&[("mode", &mode.to_string())])
            .await?;

        let result: RankingResponse = response.json().await?;
        Ok(result)
    }
}
```

### 2. Builder Pattern for Requests

```rust
// pixiv-rs-client/src/builder.rs
pub struct SearchBuilder<'a> {
    client: &'a PixivClient,
    word: Option<String>,
    sort: Option<SortOrder>,
    search_target: Option<SearchTarget>,
    duration: Option<Duration>,
    filter: Option<String>,
}

impl<'a> SearchBuilder<'a> {
    pub fn new(client: &'a PixivClient) -> Self {
        Self {
            client,
            word: None,
            sort: None,
            search_target: None,
            duration: None,
            filter: None,
        }
    }

    pub fn word(mut self, word: impl Into<String>) -> Self {
        self.word = Some(word.into());
        self
    }

    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.sort = Some(sort);
        self
    }

    pub fn target(mut self, target: SearchTarget) -> Self {
        self.search_target = Some(target);
        self
    }

    pub async fn execute(self) -> PixivResult<SearchResult> {
        let mut params = Vec::new();

        if let Some(word) = &self.word {
            params.push(("word", word.clone()));
        }
        if let Some(sort) = &self.sort {
            params.push(("sort", sort.to_string()));
        }
        // ... add other parameters

        let response = self
            .client
            .get("/v1/search/illust")
            .query(&params)
            .await?;

        Ok(response.json().await?)
    }
}

// Usage
let results = client
    .search()
    .word("landscape")
    .sort(SortOrder::PopularDesc)
    .target(SearchTarget::PartialMatchForTags)
    .execute()
    .await?;
```

## Async Patterns

### 1. Stream-based Pagination

**Python Pattern**:
```python
def paginate_search(self, word):
    result = self.search_illust(word=word)
    for item in result.illusts:
        yield item
    while result.next_url:
        next_qs = self.parse_qs(result.next_url)
        result = self.search_illust(**next_qs)
        for item in result.illusts:
            yield item
```

**Rust Implementation**:
```rust
// pixiv-rs-client/src/stream.rs
use futures::stream::{Stream, StreamExt};
use pin_project_lite::pin_project;
use std::pin::Pin;
use std::task::{Context, Poll};

pin_project! {
    pub struct PaginationStream<T> {
        #[pin]
        client: PixivClient,
        endpoint: String,
        params: Vec<(String, String)>,
        state: StreamState<T>,
    }
}

enum StreamState<T> {
    Loading,
    Ready(Vec<T>),
    Done,
}

impl<T> Stream for PaginationStream<T>
where
    T: serde::de::DeserializeOwned + 'static,
{
    type Item = PixivResult<T>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        match this.state {
            StreamState::Loading => {
                // Make API request
                // Update state with results
                Poll::Pending
            }
            StreamState::Ready(items) => {
                if let Some(item) = items.pop() {
                    Poll::Ready(Some(Ok(item)))
                } else {
                    // Check for next page
                    Poll::Pending
                }
            }
            StreamState::Done => Poll::Ready(None),
        }
    }
}

// Usage
let mut stream = client
    .search_stream()
    .word("art")
    .execute();

while let Some(Ok(illust)) = stream.next().await {
    println!("Found: {}", illust.title);
}
```

### 2. Concurrent Operations

```rust
// pixiv-rs-client/src/concurrent.rs
use tokio::task::JoinSet;
use futures::future::join_all;

pub struct ConcurrentClient {
    client: PixivClient,
    max_concurrency: usize,
}

impl ConcurrentClient {
    pub async fn get_multiple_illusts(
        &self,
        ids: Vec<u64>,
    ) -> PixivResult<Vec<IllustInfo>> {
        let mut set = JoinSet::new();

        // Split into chunks
        for chunk in ids.chunks(self.max_concurrency) {
            for &id in chunk {
                let client = self.client.clone();
                set.spawn(async move {
                    client.illust_detail(id).await
                });
            }

            // Wait for chunk to complete
            let results: Vec<_> = set.join_all().await;
            set.clear();
        }

        // Collect all results
        let mut illusts = Vec::new();
        let mut errors = Vec::new();

        for result in set.join_all().await {
            match result {
                Ok(Ok(illust)) => illusts.push(illust),
                Ok(Err(e)) => errors.push(e),
                Err(e) => errors.push(PixivError::from(e)),
            }
        }

        if errors.is_empty() {
            Ok(illusts)
        } else {
            Err(PixivError::BatchError {
                results: illusts,
                errors,
            })
        }
    }
}
```

## Feature Flags and Configuration

### 1. Cargo.toml Configuration

```toml
# pixiv-rs/Cargo.toml
[package]
name = "pixiv-rs"
version = "0.1.0"
edition = "2021"

[features]
default = ["client", "models", "tls-rustls"]
client = ["pixiv-rs-client"]
bypass = ["pixiv-rs-bypass"]
models = ["pixiv-rs-models"]
core = ["pixiv-rs-core"]
error = ["pixiv-rs-error"]
utils = ["pixiv-rs-utils"]

# TLS backends
tls-rustls = ["reqwest/rustls-tls"]
tls-native = ["reqwest/native-tls"]

# Optional features
blocking = ["reqwest/blocking"]
gzip = ["reqwest/gzip"]
brotli = ["reqwest/brotli"]
socks = ["reqwest/socks"]

[dependencies]
pixiv-rs-core = { path = "../pixiv-rs-core", optional = true }
pixiv-rs-client = { path = "../pixiv-rs-client", optional = true }
pixiv-rs-bypass = { path = "../pixiv-rs-bypass", optional = true }
pixiv-rs-models = { path = "../pixiv-rs-models", optional = true }
pixiv-rs-error = { path = "../pixiv-rs-error", optional = true }
pixiv-rs-utils = { path = "../pixiv-rs-utils", optional = true }
```

### 2. Conditional Compilation

```rust
// pixiv-rs/src/lib.rs
#[cfg(feature = "client")]
pub use pixiv_rs_client::PixivClientBuilder;

#[cfg(feature = "bypass")]
pub use pixiv_rs_bypass::ByPassSniClient;

#[cfg(feature = "models")]
pub use pixiv_rs_models::*;

pub use pixiv_rs_error::PixivError;
pub type PixivResult<T> = Result<T, PixivError>;
```

## Testing Strategy

### 1. Unit Tests

```rust
// pixiv-rs-models/tests/test_models.rs
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_illust_deserialization() {
        let json = r#"
        {
            "id": 12345678,
            "title": "Test Illustration",
            "user": {
                "id": 87654321,
                "name": "TestUser"
            },
            "createDate": "2023-01-01T12:00:00+09:00"
        }
        "#;

        let illust: IllustInfo = serde_json::from_str(json).unwrap();
        assert_eq!(illust.id, 12345678);
        assert_eq!(illust.title, "Test Illustration");
    }
}
```

### 2. Integration Tests

```rust
// tests/integration_test.rs
use pixiv_rs::PixivClientBuilder;
use tokio::test;

#[test]
async fn test_auth_and_search() {
    let client = PixivClientBuilder::new()
        .with_refresh_token(&std::env::var("PIXIV_REFRESH_TOKEN").unwrap())
        .build()
        .await;

    let result = client
        .search()
        .word("test")
        .execute()
        .await;

    assert!(result.is_ok());
}
```

### 3. Mock Server Testing

```rust
// tests/mock_server.rs
use mockito::{mock, Server};

#[tokio::test]
async fn test_with_mock_server() {
    let mut server = Server::new();

    let _m = mock("GET", "/v1/illust/detail")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"illust": {"id": 123, "title": "Test"}}"#)
        .create();

    let client = PixivClientBuilder::new()
        .with_base_url(server.url())
        .build()
        .await;

    let result = client.illust_detail(123).await;
    assert!(result.is_ok());
}
```

## Performance Optimizations

### 1. Zero-Copy Deserialization

```rust
use simd_json;

pub fn fast_parse_illusts(data: &[u8]) -> PixivResult<Vec<IllustInfo>> {
    let mut obj = simd_json::to_borrowed_value(data)?;
    let illusts = obj["illusts"].as_array_mut().unwrap();

    let mut results = Vec::with_capacity(illusts.len());
    for item in illusts {
        let illust: IllustInfo = simd_json::from_slice(item.get())?;
        results.push(illust);
    }

    Ok(results)
}
```

### 2. Connection Pooling

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

pub struct PooledClient {
    client: Client,
    semaphore: Arc<Semaphore>,
    max_concurrency: usize,
}

impl PooledClient {
    pub async fn request(&self, req: RequestBuilder) -> PixivResult<Response> {
        let _permit = self.semaphore.acquire().await;
        req.send().await.map_err(PixivError::from)
    }
}
```

## Migration Checklist

### Phase 1: Core Infrastructure
- [ ] Set up workspace structure
- [ ] Implement error types with `thiserror`
- [ ] Create base HTTP client with `reqwest`
- [ ] Define data models with `serde`
- [ ] Set up authentication flow
- [ ] Configure feature flags

### Phase 2: API Implementation
- [ ] Implement core API traits
- [ ] Add illustration endpoints
- [ ] Add user endpoints
- [ ] Add novel endpoints
- [ ] Implement search functionality
- [ ] Add pagination support

### Phase 3: Advanced Features
- [ ] Implement network bypass
- [ ] Add concurrent request support
- [ ] Implement streaming pagination
- [ ] Add file download functionality
- [ ] Implement rate limiting

### Phase 4: Testing & Documentation
- [ ] Write unit tests
- [ ] Add integration tests
- [ ] Create documentation
- [ ] Add examples
- [ ] Performance benchmarking

## Best Practices

1. **Use async/await** for all I/O operations
2. **Implement proper error handling** with `thiserror`
3. **Use `Arc` and `Clone`** for sharing client instances
4. **Leverage Rust's type system** for compile-time safety
5. **Use feature flags** for optional functionality
6. **Implement proper logging** with `tracing`
7. **Use `tokio` runtime** for async operations
8. **Write comprehensive tests** with mocks
9. **Document all public APIs** with rustdoc
10. **Consider cross-platform compatibility** for network code

## Common Pitfalls

1. **Blocking in async context** - Use async alternatives
2. **String allocations** - Prefer `&str` where possible
3. **JSON parsing overhead** - Consider zero-copy parsing
4. **Connection leaks** - Ensure proper cleanup
5. **Thread safety** - Use `Arc` and `Mutex` correctly
6. **Error handling** - Don't use `.unwrap()` in production
7. **Feature creep** - Keep API minimal and focused
8. **Documentation** - Document complex async patterns
9. **Testing** - Mock external dependencies
10. **Versioning** - Follow semantic versioning