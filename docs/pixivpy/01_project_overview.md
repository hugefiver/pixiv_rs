# PixivPy Project Overview

## Introduction

PixivPy3 is a comprehensive Python library for accessing the Pixiv API (version 3.13.0), providing developers with a complete interface to interact with Pixiv's platform programmatically. This document serves as an overview for porting the library to Rust.

## Project Scope

The library enables developers to:
- Authenticate with Pixiv using OAuth2 refresh tokens
- Access user data and profiles
- Retrieve and search illustrations and artworks
- Access novel content and metadata
- Manage bookmarks and follows
- Download images and content
- Bypass network restrictions and Cloudflare protection

## Key Features

### 1. **Authentication System**
- OAuth2 with refresh tokens (password authentication deprecated)
- Security headers (X-Client-Time, X-Client-Hash)
- Automatic token refresh
- Support for mobile app API authentication

### 2. **Comprehensive API Coverage**
- **50+ API endpoints** covering all major Pixiv functionality
- Illustration APIs (details, rankings, recommendations, comments)
- User APIs (profiles, artworks, bookmarks, follows)
- Novel APIs (content, series, comments, search)
- Search and discovery APIs

### 3. **Network Capabilities**
- Cloudflare bypass using cloudscraper
- DNS over HTTPS (DoH) support
- SNI bypass capabilities
- Proxy support (HTTP/HTTPS)
- SSL/TLS configuration options

### 4. **Data Modeling**
- Pydantic models for type safety
- Support for both Pydantic v1 and v2
- Runtime validation
- CamelCase conversion for API compatibility

### 5. **Developer Experience**
- Pagination support with URL parsing
- Content filtering (AI content, mature content)
- Rate limiting awareness
- Comprehensive error handling
- Rich examples and documentation

## Architecture Overview

### Core Components

```
PixivPy3 Architecture
├── BasePixivAPI (api.py)
│   ├── Authentication handling
│   ├── HTTP client configuration
│   ├── Request/response processing
│   └── Error handling
├── AppPixivAPI (aapi.py)
│   ├── All API endpoints implementation
│   ├── Typed response models
│   └── Pagination support
└── ByPassSniApi (bapi.py)
    ├── DNS over HTTPS
    ├── Host header SSL adapter
    └── Network bypass logic
```

### API Layers

1. **Base Layer (BasePixivAPI)**
   - Core HTTP functionality
   - Authentication management
   - Configuration handling

2. **Application Layer (AppPixivAPI)**
   - All Pixiv API endpoints
   - Data model serialization/deserialization
   - Business logic implementation

3. **Network Layer (ByPassSniApi)**
   - Specialized network handling
   - IP resolution and SNI bypass
   - Fallback mechanisms

## Supported Pixiv API Version

- **API Version**: 3.13.0
- **API Base URLs**:
  - Primary: `https://app-api.pixiv.net`
  - Bypass: Uses resolved IP addresses
  - Authentication: `https://oauth.secure.pixiv.net`

## Client Configuration

### Default User-Agent
```
PixivIOSApp/7.13.3 (iOS 14.6; iPhone13,2)
```

### Client Credentials
```python
client_id = "MOBrBDS8blbauoSck0ZfDbtuzpyT"
client_secret = "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj"
hash_secret = "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c"
```

## Authentication Flow Evolution

### Previous (Deprecated)
- Username/password authentication
- Simple token-based access

### Current Implementation
- OAuth2 with refresh tokens
- Required security headers
- X-Client-Time: Unix timestamp
- X-Client-Hash: MD5 hash of timestamp + hash_secret

## Usage Patterns

### Basic Usage
```python
from pixivpy3 import AppPixivAPI

api = AppPixivAPI()
api.auth(refresh_token="YOUR_REFRESH_TOKEN")

# Get illustration details
result = api.illust_detail(59580629)
```

### Network Bypass Usage
```python
from pixivpy3 import ByPassSniApi

api = ByPassSniApi()
api.require_appapi_hosts()  # Resolve real IPs
api.auth(refresh_token="YOUR_REFRESH_TOKEN")
```

## Project Statistics

- **50+ API endpoints** implemented
- **20+ Pydantic models** for data structures
- Support for illustrations, novels, users, and search
- Comprehensive test coverage
- Active development and maintenance

## Target Audience

The library serves:
- **Art collectors** wanting to automate downloads
- **Researchers** analyzing Pixiv data
- **Developers** building Pixiv-integrated applications
- **Data scientists** performing social media analysis

## Implementation Philosophy

1. **Type Safety**: Strong typing with Pydantic models
2. **Reliability**: Comprehensive error handling and retry logic
3. **Flexibility**: Multiple authentication and network options
4. **Performance**: Efficient HTTP client usage
5. **Compatibility**: Backward compatibility with older Python versions

## Future Considerations

For the Rust port, consider:
- Leveraging Rust's type system for even stronger guarantees
- Async/await patterns for better concurrency
- Zero-copy deserialization where possible
- Memory safety for large-scale data processing
- Integration with Rust's async ecosystem (tokio, async-std)

## Next Steps

The following documents in this series will dive deep into:
1. Authentication mechanisms and security
2. Detailed API endpoint specifications
3. Data models and structures
4. HTTP client architecture
5. Network bypass strategies
6. Error handling patterns
7. Usage patterns and examples
8. Dependencies and alternatives
9. Rust migration guide

Each section will provide concrete implementation details and code examples to guide the Rust port development.