# HTTP Client Architecture

## Overview

PixivPy's HTTP client is built around Python's `requests` library with enhancements for Cloudflare bypass and specialized network handling. This document details the HTTP client architecture, configuration, and implementation patterns for porting to Rust.

## Core HTTP Components

### 1. BasePixivAPI HTTP Configuration

```python
import requests
from requests.adapters import HTTPAdapter
from urllib3.util.retry import Retry

class BasePixivAPI:
    USER_AGENT = "PixivIOSApp/7.13.3 (iOS 14.6; iPhone13,2)"
    CLIENT_ID = "MOBrBDS8blbauoSck0ZfDbtuzpyT"
    CLIENT_SECRET = "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj"
    HASH_SECRET = "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c"

    def __init__(self, **kwargs):
        self.session = requests.Session()
        self._setup_session(**kwargs)
        self.access_token = None
        self.refresh_token = None
```

### 2. Session Configuration

```python
def _setup_session(self, proxies=None, timeout=10, verify_ssl=True):
    """Configure the HTTP session with retry strategy"""

    # Retry strategy
    retry_strategy = Retry(
        total=3,
        backoff_factor=1,
        status_forcelist=[429, 500, 502, 503, 504],
        allowed_methods=["GET", "POST", "DELETE"]
    )

    # HTTP adapter with retry
    adapter = HTTPAdapter(
        max_retries=retry_strategy,
        pool_connections=10,
        pool_maxsize=10
    )

    # Mount adapters
    self.session.mount("http://", adapter)
    self.session.mount("https://", adapter)

    # Default headers
    self.session.headers.update({
        "User-Agent": self.USER_AGENT,
        "Accept-Language": "en-us"
    })

    # Proxy configuration
    if proxies:
        self.session.proxies.update(proxies)

    # SSL verification
    if not verify_ssl:
        self.session.verify = False
        import urllib3
        urllib3.disable_warnings(urllib3.exceptions.InsecureRequestWarning)

    # Timeout
    self.timeout = timeout
```

## Request Methods Implementation

### 1. Generic GET Request

```python
def requests_call(
    self,
    method,
    url,
    params=None,
    data=None,
    headers=None,
    json_data=None,
    stream=False
):
    """Generic request method with error handling"""

    # Prepare request
    req_headers = self.session.headers.copy()
    if headers:
        req_headers.update(headers)

    # Add authentication if available
    if self.access_token:
        req_headers["Authorization"] = f"Bearer {self.access_token}"

    try:
        # Execute request
        response = self.session.request(
            method=method,
            url=url,
            params=params,
            data=data,
            json=json_data,
            headers=req_headers,
            stream=stream,
            timeout=self.timeout
        )

        # Check for errors
        if response.status_code >= 400:
            self._handle_error(response)

        # Return based on response type
        if stream:
            return response
        else:
            return response.json()

    except requests.exceptions.Timeout:
        raise PixivError("Request timeout")
    except requests.exceptions.ConnectionError:
        raise PixivError("Connection error")
    except requests.exceptions.RequestException as e:
        raise PixivError(f"Request failed: {str(e)}")
```

### 2. Simplified Methods

```python
def get(self, url, params=None, **kwargs):
    """Simplified GET request"""
    return self.requests_call("GET", url, params=params, **kwargs)

def post(self, url, data=None, **kwargs):
    """Simplified POST request"""
    return self.requests_call("POST", url, data=data, **kwargs)

def delete(self, url, **kwargs):
    """Simplified DELETE request"""
    return self.requests_call("DELETE", url, **kwargs)
```

## Cloudflare Bypass Integration

### 1. Cloudscraper Integration

```python
import cloudscraper

class BasePixivAPI:
    def __init__(self, **kwargs):
        self.use_cloudscraper = kwargs.get("cloudflare_bypass", True)
        if self.use_cloudscraper:
            self._setup_cloudscraper_session(**kwargs)
        else:
            self._setup_session(**kwargs)

    def _setup_cloudscraper_session(self, **kwargs):
        """Setup session with Cloudflare bypass"""
        self.session = cloudscraper.create_scraper(
            browser={
                'browser': 'chrome',
                'platform': 'ios',
                'desktop': False
            }
        )

        # Apply additional configuration
        self._apply_session_config(**kwargs)
```

### 2. Fallback Mechanism

```python
def _make_request_with_fallback(self, method, url, **kwargs):
    """Make request with fallback to regular requests"""
    try:
        # Try with cloudscraper first
        if self.use_cloudscraper:
            return self.requests_call(method, url, **kwargs)
        else:
            raise requests.RequestException("Cloudscraper disabled")

    except (requests.RequestException, Exception) as e:
        if self.use_cloudscraper:
            # Fallback to regular requests
            self._setup_regular_session()
            return self.requests_call(method, url, **kwargs)
        else:
            raise
```

## File Download Implementation

### 1. Download Method

```python
import os
import urllib.parse
from pathlib import Path

def download(
    self,
    url,
    path=None,
    prefix=None,
    ext=None,
    replace=False,
    referer="https://app-api.pixiv.net/"
):
    """Download file with proper referer and progress"""

    # Parse URL for filename
    parsed_url = urllib.parse.urlparse(url)
    filename = os.path.basename(parsed_url.path)

    # Determine file path
    if path:
        path_obj = Path(path)
        if path_obj.is_dir():
            filename = path_obj / filename
        else:
            filename = path_obj

    # Add prefix
    if prefix:
        filename = Path(filename).with_name(f"{prefix}{Path(filename).name}")

    # Override extension
    if ext and not ext.startswith('.'):
        ext = f".{ext}"
    if ext:
        filename = Path(filename).with_suffix(ext)

    # Check if file exists
    if not replace and Path(filename).exists():
        print(f"File already exists: {filename}")
        return str(filename)

    # Create directory if needed
    Path(filename).parent.mkdir(parents=True, exist_ok=True)

    # Download with streaming
    headers = {"Referer": referer}

    try:
        response = self.requests_call(
            "GET",
            url,
            headers=headers,
            stream=True
        )

        # Save file
        with open(filename, 'wb') as f:
            for chunk in response.iter_content(chunk_size=8192):
                if chunk:
                    f.write(chunk)

        print(f"Downloaded: {filename}")
        return str(filename)

    except Exception as e:
        raise PixivError(f"Download failed: {str(e)}")
```

### 2. Batch Download

```python
def download_batch(
    self,
    urls,
    directory=None,
    max_workers=5,
    **kwargs
):
    """Download multiple files concurrently"""

    from concurrent.futures import ThreadPoolExecutor, as_completed

    if directory:
        Path(directory).mkdir(parents=True, exist_ok=True)

    results = []

    with ThreadPoolExecutor(max_workers=max_workers) as executor:
        # Submit all download tasks
        future_to_url = {
            executor.submit(self.download, url, **kwargs): url
            for url in urls
        }

        # Collect results
        for future in as_completed(future_to_url):
            url = future_to_url[future]
            try:
                result = future.result()
                results.append((url, result, None))
            except Exception as e:
                results.append((url, None, str(e)))

    return results
```

## Connection Pooling and Performance

### 1. Connection Management

```python
def _optimize_connection_pool(self):
    """Optimize connection pool for performance"""

    # Increase pool size for concurrent requests
    adapter = HTTPAdapter(
        pool_connections=20,
        pool_maxsize=20,
        max_retries=Retry(
            total=3,
            backoff_factor=0.5,
            status_forcelist=[429, 503]
        )
    )

    self.session.mount("http://", adapter)
    self.session.mount("https://", adapter)
```

### 2. Keep-Alive Configuration

```python
def _enable_keep_alive(self):
    """Enable HTTP keep-alive for better performance"""

    from requests.packages.urllib3.connection import HTTPConnection

    # Patch connection class to add keep-alive
    class KeepAliveHTTPConnection(HTTPConnection):
        def __init__(self, *args, **kwargs):
            super().__init__(*args, **kwargs)

    # Use the patched connection
    self.session.mount('http://', HTTPAdapter())
```

## Rate Limiting

### 1. Built-in Rate Limiting

```python
import time
import threading
from collections import deque

class RateLimiter:
    def __init__(self, calls=100, period=60):
        self.calls = calls
        self.period = period
        self.times = deque()
        self.lock = threading.Lock()

    def wait(self):
        with self.lock:
            now = time.time()

            # Remove old timestamps
            while self.times and self.times[0] < now - self.period:
                self.times.popleft()

            # Check if we've exceeded the limit
            if len(self.times) >= self.calls:
                sleep_time = self.period - (now - self.times[0])
                if sleep_time > 0:
                    time.sleep(sleep_time)

            # Add current timestamp
            self.times.append(now)

# Usage in HTTP client
class BasePixivAPI:
    def __init__(self, **kwargs):
        self.rate_limiter = RateLimiter(calls=100, period=60)

    def requests_call(self, method, url, **kwargs):
        self.rate_limiter.wait()
        # ... proceed with request
```

## Error Handling in HTTP Layer

### 1. HTTP Error Types

```python
class PixivError(Exception):
    def __init__(self, reason, header=None, body=None):
        self.reason = reason
        self.header = header
        self.body = body
        super().__init__(reason)

class PixivHTTPError(PixivError):
    def __init__(self, status_code, message, response):
        self.status_code = status_code
        self.response = response
        super().__init__(f"HTTP {status_code}: {message}")

class PixivRateLimitError(PixivHTTPError):
    def __init__(self, retry_after=None, **kwargs):
        self.retry_after = retry_after
        super().__init__(status_code=429, message="Rate limited", **kwargs)

class PixivAuthError(PixivHTTPError):
    def __init__(self, **kwargs):
        super().__init__(status_code=401, message="Authentication failed", **kwargs)
```

### 2. Error Response Handling

```python
def _handle_error(self, response):
    """Handle HTTP error responses"""

    status_code = response.status_code
    message = response.reason

    # Try to get error details from body
    try:
        body = response.json()
        if body.get("error"):
            message = body["error"]["message"]
    except:
        body = response.text

    # Specific error handling
    if status_code == 401:
        raise PixivAuthError(
            header=response.headers,
            body=body
        )
    elif status_code == 429:
        retry_after = response.headers.get("Retry-After")
        raise PixivRateLimitError(
            retry_after=retry_after,
            header=response.headers,
            body=body
        )
    else:
        raise PixivHTTPError(
            status_code=status_code,
            message=message,
            response=response
        )
```

## Proxy Support

### 1. Proxy Configuration

```python
def setup_proxies(self, proxies=None):
    """Configure proxy settings"""

    if proxies is None:
        # Read from environment
        import os
        proxies = {
            'http': os.getenv('HTTP_PROXY'),
            'https': os.getenv('HTTPS_PROXY'),
            'no_proxy': os.getenv('NO_PROXY')
        }

    # Remove None values
    proxies = {k: v for k, v in proxies.items() if v}

    if proxies:
        self.session.proxies.update(proxies)
```

### 2. Proxy Authentication

```python
def setup_proxy_auth(self, proxy_user=None, proxy_pass=None):
    """Configure proxy authentication"""

    if proxy_user and proxy_pass:
        from requests.auth import HTTPProxyAuth

        self.session.auth = HTTPProxyAuth(
            proxy_user,
            proxy_pass
        )
```

## Rust Implementation Considerations

### 1. HTTP Client Selection
```rust
use reqwest::{Client, Response, RequestBuilder};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct PixivHttpClient {
    client: Client,
    access_token: Option<String>,
    rate_limiter: RateLimiter,
}

impl PixivHttpClient {
    pub fn new() -> Result<Self, PixivError> {
        let client = Client::builder()
            .user_agent("PixivIOSApp/7.13.3 (iOS 14.6; iPhone13,2)")
            .default_headers(self.default_headers())
            .timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(20)
            .pool_idle_timeout(Duration::from_secs(60))
            .build()?;

        Ok(Self {
            client,
            access_token: None,
            rate_limiter: RateLimiter::new(100, Duration::from_secs(60)),
        })
    }
}
```

### 2. Custom Cloudflare Bypass
```rust
#[cfg(feature = "cloudflare-bypass")]
use cloudflare_bypass::{CloudflareBypass, BypassConfig};

#[derive(Debug, Clone)]
pub struct PixivHttpClient {
    #[cfg(feature = "cloudflare-bypass")]
    cf_bypass: Option<CloudflareBypass>,
    // ... other fields
}

impl PixivHttpClient {
    pub async fn request_with_bypass(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> Result<Response, PixivError> {
        // Try regular request first
        let result = self.make_request(&method, url).await;

        // If it fails with Cloudflare error, try bypass
        if let Err(PixivError::CloudflareProtected) = result {
            if let Some(bypass) = &self.cf_bypass {
                let real_ip = bypass.resolve_ip(url).await?;
                return self.make_request_to_ip(&method, url, &real_ip).await;
            }
        }

        result
    }
}
```

### 3. Async Download Implementation
```rust
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

impl PixivHttpClient {
    pub async fn download(
        &self,
        url: &str,
        path: &Path,
        referer: Option<&str>,
    ) -> Result<PathBuf, PixivError> {
        let mut headers = HeaderMap::new();
        if let Some(r) = referer {
            headers.insert("Referer", r.parse()?);
        }

        let response = self
            .client
            .get(url)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(PixivError::HttpError(response.status().as_u16()));
        }

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Stream download
        let mut file = File::create(path).await?;
        let mut stream = response.bytes_stream();

        use futures_util::StreamExt;
        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
        }

        file.flush().await?;
        Ok(path.to_path_buf())
    }
}
```

### 4. Connection Pool Configuration
```rust
use reqwest::redirect::Policy;

impl PixivHttpClient {
    fn configure_client() -> ClientBuilder {
        Client::builder()
            .connection_verbose(true)
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(20)
            .tcp_keepalive(Duration::from_secs(30))
            .http2_keep_alive_interval(Duration::from_secs(15))
            .http2_keep_alive_timeout(Duration::from_secs(10))
            .http2_keep_alive_while_idle(true)
            .redirect(Policy::limited(5))
    }
}
```

### 5. Rate Limiting Implementation
```rust
use std::time::{Duration, Instant};
use tokio::time::sleep;

#[derive(Debug)]
pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    requests: Vec<Instant>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            requests: Vec::new(),
        }
    }

    pub async fn acquire(&mut self) {
        let now = Instant::now();

        // Remove old requests outside window
        self.requests.retain(|&t| now.duration_since(t) < self.window);

        // Check if we need to wait
        if self.requests.len() >= self.max_requests as usize {
            let oldest = self.requests[0];
            let wait_time = self.window - now.duration_since(oldest);
            sleep(wait_time).await;
        }

        self.requests.push(now);
    }
}
```

## Best Practices

1. **Use connection pooling** for performance
2. **Implement proper timeouts** to prevent hanging
3. **Handle rate limits gracefully** with backoff
4. **Use streaming for large downloads** to manage memory
5. **Maintain proper headers** including referer for images
6. **Implement retry logic** for transient errors
7. **Log request/response details** for debugging
8. **Validate SSL certificates** unless bypass is needed
9. **Use async/await** for non-blocking operations
10. **Monitor connection metrics** for performance optimization