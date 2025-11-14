# Error Handling Patterns

## Overview

PixivPy implements a comprehensive error handling system that provides detailed context for different types of failures. This document details the error types, handling patterns, and recovery strategies for porting to Rust.

## Error Hierarchy

### 1. Base Exception Class

```python
class PixivError(Exception):
    """Base exception for all PixivPy errors"""

    def __init__(self, reason, header=None, body=None, response=None):
        self.reason = reason
        self.header = header
        self.body = body
        self.response = response
        super().__init__(reason)

    def __str__(self):
        """String representation with context"""
        message = f"PixivError: {self.reason}"
        if self.header:
            message += f"\nHeaders: {self.header}"
        if self.body:
            message += f"\nBody: {self.body}"
        return message

    def __repr__(self):
        return f"PixivError('{self.reason}')"
```

### 2. Specific Error Types

```python
class PixivAuthError(PixivError):
    """Authentication-related errors"""
    def __init__(self, reason="Authentication failed", **kwargs):
        super().__init__(reason, **kwargs)

class PixivRateLimitError(PixivError):
    """Rate limit exceeded errors"""
    def __init__(self, reason="Rate limit exceeded", retry_after=None, **kwargs):
        self.retry_after = retry_after
        super().__init__(reason, **kwargs)

class PixivNotFoundError(PixivError):
    """Resource not found errors"""
    def __init__(self, reason="Resource not found", resource_id=None, **kwargs):
        self.resource_id = resource_id
        super().__init__(reason, **kwargs)

class PixivNetworkError(PixivError):
    """Network connectivity errors"""
    def __init__(self, reason="Network error", original_error=None, **kwargs):
        self.original_error = original_error
        super().__init__(reason, **kwargs)

class PixivValidationError(PixivError):
    """Parameter validation errors"""
    def __init__(self, reason="Validation failed", field=None, **kwargs):
        self.field = field
        super().__init__(reason, **kwargs)

class PixivJSONError(PixivError):
    """JSON parsing errors"""
    def __init__(self, reason="JSON parsing failed", **kwargs):
        super().__init__(reason, **kwargs)
```

## HTTP Error Handling

### 1. Response Error Detection

```python
def _handle_error_response(self, response):
    """Handle HTTP error responses"""

    status_code = response.status_code
    url = response.url
    headers = response.headers

    # Parse response body
    try:
        body = response.json()
    except:
        body = response.text

    # Determine error type based on status code
    if status_code == 400:
        raise PixivValidationError(
            reason="Bad request - invalid parameters",
            header=dict(headers),
            body=body,
            response=response
        )

    elif status_code == 401:
        raise PixivAuthError(
            reason="Unauthorized - invalid or expired token",
            header=dict(headers),
            body=body,
            response=response
        )

    elif status_code == 403:
        raise PixivAuthError(
            reason="Forbidden - insufficient permissions",
            header=dict(headers),
            body=body,
            response=response
        )

    elif status_code == 404:
        raise PixivNotFoundError(
            reason="Resource not found",
            header=dict(headers),
            body=body,
            response=response
        )

    elif status_code == 429:
        retry_after = headers.get('Retry-After')
        raise PixivRateLimitError(
            reason="Rate limit exceeded",
            retry_after=retry_after,
            header=dict(headers),
            body=body,
            response=response
        )

    elif status_code >= 500:
        raise PixivNetworkError(
            reason=f"Server error: {status_code}",
            header=dict(headers),
            body=body,
            response=response
        )

    else:
        raise PixivError(
            reason=f"HTTP {status_code}: {response.reason}",
            header=dict(headers),
            body=body,
            response=response
        )
```

### 2. Network Error Detection

```python
import requests
from urllib3.exceptions import MaxRetryError, ProtocolError

def _handle_network_error(self, error):
    """Handle network-level errors"""

    if isinstance(error, requests.exceptions.Timeout):
        raise PixivNetworkError(
            reason="Request timeout",
            original_error=error
        )

    elif isinstance(error, requests.exceptions.ConnectionError):
        if "Connection refused" in str(error):
            raise PixivNetworkError(
                reason="Connection refused - service unavailable",
                original_error=error
            )
        elif "Name resolution failed" in str(error):
            raise PixivNetworkError(
                reason="DNS resolution failed",
                original_error=error
            )
        elif "SSL" in str(error) or "certificate" in str(error):
            raise PixivNetworkError(
                reason="SSL/TLS error",
                original_error=error
            )
        else:
            raise PixivNetworkError(
                reason="Connection error",
                original_error=error
            )

    elif isinstance(error, requests.exceptions.RequestException):
        raise PixivNetworkError(
            reason="Request failed",
            original_error=error
        )

    else:
        raise PixivError(
            reason=f"Unexpected error: {type(error).__name__}",
            original_error=error
        )
```

## API Response Validation

### 1. Response Structure Validation

```python
def _validate_response(self, response_json, endpoint):
    """Validate API response structure"""

    # Check for error in response body
    if isinstance(response_json, dict):
        if "error" in response_json:
            error_info = response_json["error"]
            if isinstance(error_info, dict):
                message = error_info.get("message", "Unknown API error")
                raise PixivError(
                    reason=f"API error: {message}",
                    body=response_json
                )
            else:
                raise PixivError(
                    reason=f"API error: {error_info}",
                    body=response_json
                )

        # Check meta field for errors
        if "meta" in response_json:
            meta = response_json["meta"]
            if meta.get("status") != 200:
                error_message = meta.get("error", "Unknown error")
                raise PixivError(
                    reason=f"API error: {error_message}",
                    body=response_json
                )

    # Endpoint-specific validation
    self._validate_endpoint_response(response_json, endpoint)

def _validate_endpoint_response(self, response, endpoint):
    """Validate specific endpoint responses"""

    if endpoint == "illust_detail":
        if "illust" not in response:
            raise PixivNotFoundError(
                reason="Illustration not found",
                body=response
            )
    elif endpoint == "user_detail":
        if "user" not in response:
            raise PixivNotFoundError(
                reason="User not found",
                body=response
            )
    elif endpoint == "novel_detail":
        if "novel" not in response:
            raise PixivNotFoundError(
                reason="Novel not found",
                body=response
            )
```

### 2. Parameter Validation

```python
def _validate_parameters(self, params, endpoint):
    """Validate request parameters"""

    if endpoint == "illust_detail":
        if "illust_id" not in params:
            raise PixivValidationError(
                reason="illust_id is required",
                field="illust_id"
            )
        elif not isinstance(params["illust_id"], int) or params["illust_id"] <= 0:
            raise PixivValidationError(
                reason="illust_id must be a positive integer",
                field="illust_id"
            )

    elif endpoint == "search_illust":
        if "word" in params:
            if not params["word"] or not isinstance(params["word"], str):
                raise PixivValidationError(
                    reason="word must be a non-empty string",
                    field="word"
                )
            elif len(params["word"]) > 100:
                raise PixivValidationError(
                    reason="word is too long (max 100 characters)",
                    field="word"
                )

    elif endpoint in ["user_following", "user_bookmarks_illust"]:
        if "restrict" in params and params["restrict"] not in ["public", "private"]:
            raise PixivValidationError(
                reason="restrict must be 'public' or 'private'",
                field="restrict"
            )
```

## Error Recovery Strategies

### 1. Automatic Retry Logic

```python
import time
import random
from functools import wraps

def with_retry(max_retries=3, backoff_factor=1, retry_on=None):
    """Decorator for automatic retry with exponential backoff"""
    if retry_on is None:
        retry_on = (PixivNetworkError, PixivRateLimitError)

    def decorator(func):
        @wraps(func)
        def wrapper(self, *args, **kwargs):
            last_error = None

            for attempt in range(max_retries + 1):
                try:
                    return func(self, *args, **kwargs)

                except retry_on as e:
                    last_error = e

                    # Don't retry on last attempt
                    if attempt == max_retries:
                        break

                    # Calculate delay
                    if isinstance(e, PixivRateLimitError) and e.retry_after:
                        delay = float(e.retry_after)
                    else:
                        # Exponential backoff with jitter
                        base_delay = backoff_factor * (2 ** attempt)
                        jitter = random.uniform(0, 0.5)
                        delay = base_delay + jitter

                    # Wait before retry
                    time.sleep(delay)

                    # For rate limit, try refreshing token
                    if isinstance(e, PixivRateLimitError):
                        self._maybe_refresh_token()

                except PixivAuthError:
                    # Don't retry auth errors unless we can refresh
                    if self._can_refresh_token():
                        self._refresh_token()
                        continue
                    else:
                        raise

            raise last_error

        return wrapper
    return decorator

# Usage
@with_retry(max_retries=3, backoff_factor=1)
def illust_detail(self, illust_id):
    return self.get(f"/v1/illust/detail", params={"illust_id": illust_id})
```

### 2. Token Refresh Handling

```python
def _maybe_refresh_token(self):
    """Check and refresh token if needed"""
    if self._is_token_expired():
        self._refresh_token()

def _is_token_expired(self):
    """Check if access token is expired"""
    if not self.token_expires_at:
        return False
    return time.time() >= self.token_expires_at

def _refresh_token(self):
    """Refresh the access token"""
    try:
        auth_info = self.auth(refresh_token=self.refresh_token)
        self.access_token = auth_info["access_token"]
        self.token_expires_at = time.time() + auth_info["expires_in"]
        return True
    except PixivError:
        # Refresh failed, token might be revoked
        self.access_token = None
        self.token_expires_at = None
        return False
```

### 3. Fallback Mechanisms

```python
def get_with_fallback(self, endpoint, primary_url, fallback_url=None, **kwargs):
    """Try primary URL first, then fallback if needed"""

    try:
        return self.get(primary_url, **kwargs)
    except (PixivNetworkError, PixivRateLimitError) as e:
        if fallback_url and self._should_use_fallback(e):
            print(f"Primary failed, trying fallback: {e}")
            return self.get(fallback_url, **kwargs)
        else:
            raise

def _should_use_fallback(self, error):
    """Determine if fallback should be used"""
    # Use fallback for DNS or connection errors
    if isinstance(error, PixivNetworkError):
        return "dns" in error.reason.lower() or "connection" in error.reason.lower()
    return False
```

## Error Context and Logging

### 1. Error Context Builder

```python
class ErrorContext:
    """Build detailed error context for debugging"""

    def __init__(self):
        self.context = {}

    def add_request(self, method, url, headers=None, params=None):
        self.context.update({
            "request": {
                "method": method,
                "url": url,
                "headers": headers,
                "params": params
            }
        })
        return self

    def add_response(self, status_code, headers=None, body=None):
        self.context.update({
            "response": {
                "status_code": status_code,
                "headers": headers,
                "body": body
            }
        })
        return self

    def add_user_info(self, user_id, has_token):
        self.context.update({
            "user": {
                "id": user_id,
                "authenticated": bool(has_token)
            }
        })
        return self

    def add_timestamp(self):
        self.context["timestamp"] = time.time()
        return self

    def build(self):
        return self.context

# Usage
try:
    result = self.get(endpoint)
except PixivError as e:
    context = ErrorContext()\
        .add_request("GET", url, headers, params)\
        .add_response(e.response.status_code if e.response else None,
                     dict(e.header) if e.header else None,
                     e.body)\
        .add_user_info(self.user_id, bool(self.access_token))\
        .add_timestamp()\
        .build()

    logger.error(f"Pixiv API error: {e}", extra=context)
    raise
```

### 2. Error Logging

```python
import logging

logger = logging.getLogger("pixivpy")

class PixivLogger:
    """Custom logger for Pixiv operations"""

    @staticmethod
    def log_error(error, context=None):
        """Log error with context"""
        log_data = {
            "error_type": type(error).__name__,
            "error_message": str(error)
        }

        if context:
            log_data.update(context)

        logger.error("Pixiv error occurred", extra={"data": log_data})

    @staticmethod
    def log_rate_limit(limit_remaining, limit_reset):
        """Log rate limit status"""
        logger.info(f"Rate limit: {limit_remaining} remaining, resets at {limit_reset}")

    @staticmethod
    def log_auth_event(event, user_id=None):
        """Log authentication events"""
        logger.info(f"Auth event: {event} for user {user_id}")
```

## Rust Implementation Strategy

### 1. Error Type Definition

```rust
use thiserror::Error;
use reqwest::StatusCode;
use std::time::Duration;

#[derive(Debug, Error)]
pub enum PixivError {
    #[error("Authentication failed: {message}")]
    AuthError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Rate limit exceeded. Retry after: {retry_after:?}")]
    RateLimitError {
        retry_after: Option<Duration>,
    },

    #[error("Resource not found: {resource}")]
    NotFoundError {
        resource: String,
        id: Option<String>,
    },

    #[error("Network error: {message}")]
    NetworkError {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    #[error("Validation error in field '{field}': {message}")]
    ValidationError {
        field: String,
        message: String,
    },

    #[error("JSON parsing error: {message}")]
    JsonError {
        message: String,
        #[source]
        source: serde_json::Error,
    },

    #[error("HTTP {status}: {message}")]
    HttpError {
        status: StatusCode,
        message: String,
    },

    #[error("Configuration error: {message}")]
    ConfigError {
        message: String,
    },

    #[error("Token expired or invalid")]
    TokenExpired,

    #[error("Unknown error: {message}")]
    Unknown {
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },
}
```

### 2. Error Context Structure

```rust
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorContext {
    pub request: Option<RequestContext>,
    pub response: Option<ResponseContext>,
    pub user: Option<UserContext>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RequestContext {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub params: Option<HashMap<String, String>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResponseContext {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserContext {
    pub id: Option<u64>,
    pub authenticated: bool,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            request: None,
            response: None,
            user: None,
            timestamp: chrono::Utc::now(),
        }
    }
}
```

### 3. Result Type and Error Handling

```rust
pub type PixivResult<T> = Result<T, PixivError>;

pub trait PixivErrorExt<T> {
    fn with_context(self, context: ErrorContext) -> PixivResult<T>;
    fn with_retry_context(self, attempt: u32) -> PixivResult<T>;
}

impl<T> PixivErrorExt<T> for PixivResult<T> {
    fn with_context(self, context: ErrorContext) -> PixivResult<T> {
        self.map_err(|e| {
            // Store context with error (if using a logging library that supports it)
            error!("Pixiv error with context: {}", e; "context" => ?context);
            e
        })
    }

    fn with_retry_context(self, attempt: u32) -> PixivResult<T> {
        self.map_err(|e| {
            warn!("Request failed on attempt {}: {}", attempt, e);
            e
        })
    }
}
```

### 4. Retry Mechanism

```rust
use tokio::time::sleep;
use futures::future::FutureExt;

pub struct RetryConfig {
    pub max_retries: u32,
    pub base_delay: Duration,
    pub max_delay: Duration,
    pub backoff_factor: f64,
    pub jitter: bool,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(1000),
            max_delay: Duration::from_millis(30000),
            backoff_factor: 2.0,
            jitter: true,
        }
    }
}

pub async fn retry_with_backoff<F, T, E>(
    operation: F,
    config: RetryConfig,
) -> Result<T, E>
where
    F: Fn() -> futures::future::BoxFuture<'static, Result<T, E>>,
    E: std::error::Error + Send + Sync + 'static,
{
    let mut last_error = None;

    for attempt in 0..=config.max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(error) => {
                last_error = Some(error);

                // Don't sleep on the last attempt
                if attempt == config.max_retries {
                    break;
                }

                // Calculate delay
                let delay = calculate_delay(attempt, &config);

                // Log retry
                warn!("Attempt {} failed, retrying in {:?}", attempt + 1, delay);

                // Wait before retry
                sleep(delay).await;
            }
        }
    }

    Err(last_error.unwrap())
}

fn calculate_delay(attempt: u32, config: &RetryConfig) -> Duration {
    let delay = config.base_delay * (config.backoff_factor.powi(attempt as i32));

    let delay = if config.jitter {
        let jitter = (rand::random::<f64>() - 0.5) * 0.1;
        let multiplier = 1.0 + jitter;
        Duration::from_millis((delay.as_millis() as f64 * multiplier) as u64)
    } else {
        delay
    };

    delay.min(config.max_delay)
}
```

### 5. Error Recovery Trait

```rust
#[async_trait]
pub trait ErrorRecovery {
    async fn handle_error(&self, error: &PixivError) -> Option<PixivResult<()>> {
        match error {
            PixivError::TokenExpired => self.refresh_token().await.ok().map(Ok),
            PixivError::RateLimitError { retry_after } => {
                if let Some(duration) = retry_after {
                    sleep(*duration).await;
                    Some(Ok(()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    async fn refresh_token(&self) -> PixivResult<()>;

    fn is_retryable(&self, error: &PixivError) -> bool {
        matches!(
            error,
            PixivError::NetworkError { .. }
                | PixivError::RateLimitError { .. }
                | PixivError::HttpError { status, .. }
                    if *status == StatusCode::INTERNAL_SERVER_ERROR
                        || *status == StatusCode::BAD_GATEWAY
                        || *status == StatusCode::SERVICE_UNAVAILABLE
        )
    }
}
```

## Best Practices

1. **Provide detailed error context** for debugging
2. **Use specific error types** for different failure modes
3. **Implement exponential backoff** for retries
4. **Log errors with appropriate levels**
5. **Handle rate limits gracefully**
6. **Validate inputs early** to prevent API errors
7. **Use structured logging** for better error tracking
8. **Implement circuit breakers** for repeated failures
9. **Provide user-friendly messages** where applicable
10. **Monitor error patterns** for proactive fixes