# Dependencies & Libraries

## Overview

PixivPy relies on a set of Python libraries for HTTP communication, data validation, Cloudflare bypass, and other functionalities. This document details all dependencies and their Rust equivalents for the port.

## Core Dependencies

### 1. HTTP Client - Requests
```python
# Python
requests >= 2.31.0
```

**Purpose**: Core HTTP client for API communication
- HTTP/HTTPS requests
- Session management
- Connection pooling
- Cookie handling
- Redirect following

**Rust Equivalents**:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "cookies", "stream"] }
# or hyper + http-body for lower-level control
hyper = { version = "0.14", features = ["full"] }
http-body = "0.4"
```

**Recommended**: `reqwest` - Provides high-level API similar to requests

### 2. Data Validation - Pydantic
```python
# Python
pydantic >= 1.9.0, < 3.0.0
```

**Purpose**: Data validation and serialization
- Type hints validation
- JSON serialization/deserialization
- Runtime type checking
- Field validation

**Rust Equivalents**:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
# For datetime handling
chrono = { version = "0.4", features = ["serde"] }
# For optional validation
validator = { version = "0.16", features = ["derive"] }
```

### 3. Cloudflare Bypass - Cloudscraper
```python
# Python
cloudscraper >= 1.2.58
```

**Purpose**: Bypass Cloudflare protection
- JavaScript challenge solving
- Browser emulation
- Cookie management
- User agent rotation

**Rust Equivalents**:
```toml
[dependencies]
# No direct equivalent, custom implementation needed
# Use a combination of:
headless_chrome = "0.9"          # For JS challenges
scraper = "0.15"                # HTML parsing
reqwest = { version = "0.11" }   # HTTP client
# Or implement custom DoH-based bypass
```

### 4. SSL/TLS Utils - Requests-Toolbelt
```python
# Python
requests-toolbelt >= 1.0.0
```

**Purpose**: HTTP utility functions
- Host header SSL adapter
- Streaming uploads
- Multipart form data

**Rust Equivalents**:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["stream", "multipart"] }
# Custom SNI bypass implementation
rustls = "0.20"
webpki-roots = "0.22"
```

### 5. URLLib3 (Transitive Dependency)
```python
# Python
urllib3 >= 2.0.7
```

**Purpose**: Lower-level HTTP functionality
- Connection pooling
- SSL/TLS handling
- Retry mechanisms

**Rust Equivalents**:
```toml
[dependencies]
hyper = "0.14"
rustls = "0.20"
tokio = { version = "1.0", features = ["net", "rt-multi-thread"] }
```

## Optional Dependencies

### 1. Async Support - aiohttp
```python
# Python (optional)
aiohttp >= 3.8.0
```

**Purpose**: Async HTTP requests

**Rust Equivalents**:
```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json", "stream"] }
```

### 2. Image Processing - Pillow
```python
# Python (for examples)
Pillow >= 9.0.0
```

**Purpose**: Image manipulation

**Rust Equivalents**:
```toml
[dependencies]
image = "0.24"
imageproc = "0.23"
```

### 3. Progress Bars - tqdm
```python
# Python (examples)
tqdm >= 4.64.0
```

**Purpose**: Progress bars for downloads

**Rust Equivalents**:
```toml
[dependencies]
indicatif = "0.17"
```

### 4. Concurrent Operations - ThreadPoolExecutor
```python
# Python (built-in)
concurrent.futures
```

**Purpose**: Thread pool execution

**Rust Equivalents**:
```toml
[dependencies]
tokio = { version = "1.0", features = ["rt-multi-thread"] }
rayon = "1.5"  # For CPU parallelism
```

## Development Dependencies

### 1. Testing Framework
```python
# Python
pytest >= 7.0.0
pytest-asyncio >= 0.21.0
```

**Rust Equivalents**:
```toml
[dev-dependencies]
tokio-test = "0.4"
test-case = "2.2"
```

### 2. Code Quality
```python
# Python
black >= 22.0.0      # Formatting
flake8 >= 5.0.0      # Linting
mypy >= 1.0.0        # Type checking
```

**Rust Equivalents**:
```toml
[dev-dependencies]
rustfmt = "1.0"       # Formatting
clippy = "0.1"         # Linting (built-in)
```

## Dependency Analysis by Module

### Authentication Module
```python
# Required
requests >= 2.31.0
hashlib           # Built-in
time              # Built-in
```

**Rust Implementation**:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
sha2 = "0.10"           # For hashing
chrono = { version = "0.4", features = ["serde"] }
hex = "0.4"             # For hex encoding
```

### Data Models Module
```python
# Required
pydantic >= 1.9.0
typing            # Built-in
datetime          # Built-in
```

**Rust Implementation**:
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
uuid = { version = "1.0", features = ["serde"] }
url = { version = "2.0", features = ["serde"] }
```

### HTTP Client Module
```python
# Required
requests >= 2.31.0
requests-toolbelt >= 1.0.0
cloudscraper >= 1.2.58
```

**Rust Implementation**:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json", "cookies", "stream"] }
tokio = { version = "1.0", features = ["rt-multi-thread", "net", "time"] }
rustls = "0.20"                    # For custom SSL
webpki-roots = "0.22"              # Certificates
```

### Network Bypass Module
```python
# Required
requests-toolbelt >= 1.0.0
cloudscraper >= 1.2.58
dns.resolver      # From dnspython
```

**Rust Implementation**:
```toml
[dependencies]
reqwest = { version = "0.11", features = ["json"] }
trust-dns-resolver = "0.22"         # DNS resolution
rustls = "0.20"                     # TLS
tokio = { version = "1.0", features = ["dns"] }
ipnetwork = "0.20"                   # IP manipulation
```

## Version Management

### Python (pyproject.toml)
```toml
[project]
dependencies = [
    "requests>=2.31.0",
    "cloudscraper>=1.2.58",
    "requests-toolbelt>=1.0.0",
    "pydantic>=1.9.0,<3.0.0",
    "urllib3>=2.0.7",
]
```

### Rust (Cargo.toml)
```toml
[package]
version = "0.1.0"

[dependencies]
# Core HTTP client
reqwest = { version = "0.11", features = ["json", "cookies", "stream"] }
tokio = { version = "1.0", features = ["rt-multi-thread", "net", "time"] }

# Data serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Date/time
chrono = { version = "0.4", features = ["serde"] }

# Security/crypto
sha2 = "0.10"
hex = "0.4"

# DNS/Network
trust-dns-resolver = "0.22"

# TLS (custom implementation)
rustls = "0.20"
webpki-roots = "0.22"

# Error handling
thiserror = "1.0"
anyhow = "1.0"

# Utilities
url = { version = "2.0", features = ["serde"] }
uuid = { version = "1.0", features = ["serde"] }

# Progress (optional)
indicatif = { version = "0.17", optional = true }

# Image processing (optional)
image = { version = "0.24", optional = true }

# Logging
tracing = "0.1"
tracing-subscriber = "0.3"

[features]
default = ["progress", "image"]
progress = ["indicatif"]
image = ["dep:image"]
cloudflare-bypass = []
```

## Feature Flags Strategy

### Required Features
- `json`: JSON parsing/serialization
- `cookies`: Cookie management
- `stream`: Streaming downloads

### Optional Features
```toml
[features]
default = ["http2", "brotli", "deflate"]

# HTTP compression
brotli = ["reqwest/brotli"]
deflate = ["reqwest/deflate"]
gzip = ["reqwest/gzip"]

# HTTP/2 support
http2 = ["reqwest/http2"]

# Native TLS
native-tls = ["reqwest/native-tls"]

# Rustls TLS
rustls-tls = ["reqwest/rustls-tls"]

# WebSocket support
websocket = []

# Cloudflare bypass
cloudflare-bypass = ["headless_chrome"]

# Extended logging
tracing-log = ["tracing/log"]

# Image processing
image-support = ["image", "imageproc"]

# Progress indicators
progress = ["indicatif"]

# Async utilities
async-utils = ["futures-util"]

# DNS over HTTPS
doh = ["trust-dns-resolver/dns-over-https-rustls"]
```

## Library Comparisons

### HTTP Clients

| Python | Rust | Notes |
|--------|------|-------|
| requests | reqwest | High-level, similar API |
| aiohttp | hyper+tokio | Low-level, async-only |
| urllib3 | hyper | Low-level, building blocks |

### Data Validation

| Python | Rust | Notes |
|--------|------|-------|
| pydantic | serde | Compile-time vs runtime |
| marshmallow | serde_json | Similar functionality |
| dataclasses | serde_derive | Built-in vs derive macro |

### DNS Resolution

| Python | Rust | Notes |
|--------|------|-------|
| dnspython | trust-dns | Full DNS client |
| system DNS | tokio::net | Basic resolution |

### Cloudflare Bypass

| Python | Rust | Notes |
|--------|------|-------|
| cloudscraper | No direct equivalent | Custom implementation needed |
| playwright-rust | Browser automation | Alternative approach |
| headless_chrome | Chrome automation | For JS challenges |

## Performance Considerations

### Memory Usage
- **Python**: Higher overhead due to interpreter
- **Rust**: Zero-copy deserialization possible
- **Recommendation**: Use `serde_json::from_str` or `simd-json` for JSON parsing

### CPU Performance
- **Python**: Slower due to GIL limitations
- **Rust**: True parallelism with Rayon
- **Recommendation**: Use Rayon for CPU-bound operations

### Network I/O
- **Python**: Limited by GIL for many connections
- **Rust**: Async I/O with Tokio scales better
- **Recommendation**: Use Tokio runtime with connection pooling

## Migration Checklist

### Core Dependencies
- [ ] Replace `requests` with `reqwest`
- [ ] Replace `pydantic` with `serde`
- [ ] Implement custom Cloudflare bypass
- [ ] Replace `urllib3` utilities with native Rust

### Optional Dependencies
- [ ] Consider `tokio` for async operations
- [ ] Use `indicatif` for progress bars
- [ ] Evaluate `image` crate for image processing
- [ ] Use `rayon` for parallel processing

### Build Configuration
- [ ] Configure feature flags
- [ ] Set up proper TLS backend
- [ ] Configure async runtime
- [ ] Set up logging infrastructure

## Best Practices

1. **Pin major versions** for stability
2. **Use feature flags** to minimize dependencies
3. **Prefer Rust-native solutions** over Python ports
4. **Benchmark performance** critical paths
5. **Monitor dependency vulnerabilities** with tools like `cargo audit`
6. **Keep dependencies minimal** for faster compilation
7. **Use workspace crates** for large projects
8. **Document optional features** clearly
9. **Test with minimum versions** for compatibility
10. **Review security advisories** regularly