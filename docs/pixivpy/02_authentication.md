# Authentication System

## Overview

PixivPy uses OAuth2 authentication with refresh tokens as the primary authentication method. The password-based authentication has been deprecated. This document details the authentication flow, security requirements, and implementation details for porting to Rust.

## Authentication Methods

### 1. OAuth2 with Refresh Token (Current)
```python
api = AppPixivAPI()
api.auth(refresh_token="YOUR_REFRESH_TOKEN")
```

### 2. OAuth2 with Refresh Token + Device ID
```python
api.auth(refresh_token="YOUR_REFRESH_TOKEN", device_token="DEVICE_TOKEN")
```

### 3. Deprecated: Username/Password
```python
# NO LONGER SUPPORTED
api.login("USERNAME", "PASSWORD")
```

## OAuth2 Flow Details

### Step 1: Initial Authentication
The user must obtain a refresh token through:
- Pixiv's official OAuth flow
- Browser authentication
- Third-party OAuth clients

### Step 2: Token Exchange
Using the refresh token to obtain an access token:
- Endpoint: `https://oauth.secure.pixiv.net/auth/token`
- Method: POST
- Content-Type: `application/x-www-form-urlencoded`

### Step 3: Access Token Usage
- Access tokens are used in API calls via `Authorization: Bearer {access_token}`
- Tokens have limited lifetime (typically 3600 seconds)
- Automatic refresh using the refresh token

## Security Headers

### Required Headers for Authentication
All authentication requests must include:

```http
X-Client-Time: 1234567890
X-Client-Hash: md5(timestamp + hash_secret)
```

### Implementation Details
```python
import time
import hashlib

def generate_client_headers():
    timestamp = str(int(time.time()))
    md5_hash = hashlib.md5(f"{timestamp}{hash_secret}".encode()).hexdigest()
    return {
        "X-Client-Time": timestamp,
        "X-Client-Hash": md5_hash
    }
```

### Constants
```python
client_id = "MOBrBDS8blbauoSck0ZfDbtuzpyT"
client_secret = "lsACyCD94FhDUtGTXi3QzcFE2uU1hqtDaKeqrdwj"
hash_secret = "28c1fdd170a5204386cb1313c7077b34f83e4aaf4aa829ce78c231e05b0bae2c"
```

## Authentication API Endpoint

### Request Format
```http
POST https://oauth.secure.pixiv.net/auth/token
Content-Type: application/x-www-form-urlencoded
User-Agent: PixivIOSApp/7.13.3 (iOS 14.6; iPhone13,2)

X-Client-Time: {timestamp}
X-Client-Hash: {md5_hash}

client_id={client_id}&client_secret={client_secret}&grant_type=refresh_token&refresh_token={refresh_token}
```

### Response Format
```json
{
    "access_token": "aCCESS_TOKEN_HERE",
    "expires_in": 3600,
    "token_type": "Bearer",
    "scope": "",
    "refresh_token": "nEW_REFRESH_TOKEN_HERE",
    "user": {
        "profile_image_urls": {
            "medium": "https://s.pximg.net/common/images/no_profile.png"
        },
        "id": 12345678,
        "name": "username",
        "account": "username",
        "mail_address": "user@example.com",
        "is_premium": false,
        "x_restrict": 0,
        "has_mail": false,
        "require_agreement": false
    }
}
```

## Token Management

### Access Token
- Lifetime: 3600 seconds (1 hour)
- Used for API requests
- Must be included in `Authorization` header

### Refresh Token
- Longer lifetime
- Used to obtain new access tokens
- Can be rotated by the server
- Should be stored securely

### Token Storage Pattern
```python
class TokenManager:
    def __init__(self):
        self.access_token = None
        self.refresh_token = None
        self.expires_at = None

    def is_access_token_valid(self):
        if not self.access_token or not self.expires_at:
            return False
        return time.time() < self.expires_at

    def refresh_if_needed(self):
        if not self.is_access_token_valid():
            self.refresh_tokens()
```

## Authentication Implementation in BasePixivAPI

### Core Method: `auth()`
```python
def auth(self, refresh_token=None, device_token=None):
    """
    Authenticate with refresh token

    Args:
        refresh_token: The refresh token obtained from OAuth flow
        device_token: Optional device token for additional security

    Returns:
        Authentication response containing tokens and user info

    Raises:
        PixivError: If authentication fails
    """
```

### Authentication Flow Code
```python
def auth(self, refresh_token=None, device_token=None):
    if not refresh_token:
        raise PixivError("Refresh token is required")

    # Prepare headers
    headers = self.generate_client_headers()
    headers["User-Agent"] = self.USER_AGENT

    # Prepare form data
    data = {
        "client_id": self.client_id,
        "client_secret": self.client_secret,
        "grant_type": "refresh_token",
        "refresh_token": refresh_token,
    }

    if device_token:
        data["device_token"] = device_token

    # Make authentication request
    response = self.session.post(
        self.AUTH_URL,
        data=data,
        headers=headers
    )

    if response.status_code != 200:
        raise PixivError("Authentication failed", response.headers, response.text)

    auth_info = response.json()

    # Store tokens
    self.access_token = auth_info["access_token"]
    self.refresh_token = auth_info.get("refresh_token", refresh_token)
    self.expires_in = auth_info["expires_in"]
    self.user_info = auth_info["user"]

    # Update session headers for future requests
    self.session.headers.update({
        "Authorization": f"Bearer {self.access_token}"
    })

    return auth_info
```

## API Request Authentication

### Bearer Token Header
All API requests after authentication must include:
```http
Authorization: Bearer {access_token}
```

### Example Request
```python
def make_authenticated_request(self, endpoint):
    headers = {
        "Authorization": f"Bearer {self.access_token}",
        "User-Agent": self.USER_AGENT,
        "Accept-Language": "en-us"
    }

    response = self.session.get(endpoint, headers=headers)
    return response.json()
```

## Error Handling

### Authentication Errors

#### 400 Bad Request
```json
{
    "error": "invalid_request",
    "error_description": "Missing required parameter"
}
```

#### 401 Unauthorized
```json
{
    "error": "invalid_grant",
    "error_description": "Invalid refresh token"
}
```

#### 403 Forbidden
```json
{
    "error": "invalid_client",
    "error_description": "Invalid client credentials"
}
```

### Error Handling Pattern
```python
try:
    auth_info = api.auth(refresh_token=token)
except PixivError as e:
    if "invalid_grant" in str(e):
        # Refresh token invalid, user must re-authenticate
        print("Please obtain a new refresh token")
    else:
        # Other authentication error
        print(f"Authentication failed: {e}")
```

## Device Tokens

### Optional Device Token
- Generated per device/user combination
- Additional security layer
- Persists across sessions
- Can be stored and reused

### Device Token Generation
```python
import uuid
import hashlib

def generate_device_token(user_id=None):
    """
    Generate a device token for additional security

    Args:
        user_id: Optional user ID for deterministic generation

    Returns:
        String device token
    """
    if user_id:
        # Deterministic token based on user ID
        data = f"{user_id}{hash_secret}".encode()
    else:
        # Random token
        data = f"{uuid.uuid4()}{hash_secret}".encode()

    return hashlib.sha256(data).hexdigest()
```

## Security Considerations

### 1. Token Storage
- Store refresh tokens securely (encrypted storage)
- Consider using OS keychain/secure storage
- Never log or expose tokens

### 2. Hash Secret
- Keep the hash secret secure in the client
- Do not expose in logs or error messages
- Consider obfuscation in production builds

### 3. Request Validation
- Always include required security headers
- Validate token expiration before use
- Implement proper error handling for expired tokens

### 4. Network Security
- Use HTTPS for all requests
- Validate SSL certificates (except for bypass scenarios)
- Consider certificate pinning for additional security

## Rate Limiting

### Authentication Rate Limits
- Too many failed attempts may trigger temporary bans
- Implement exponential backoff for retries
- Monitor authentication success/failure rates

### API Rate Limits
- Authenticated requests have higher limits
- Rate limit information in response headers
- Implement polite request patterns

## Migration to Rust: Key Considerations

### 1. HTTP Client
- Use `reqwest` for HTTP requests
- Configure default headers properly
- Handle cookies and sessions

### 2. Time Management
- Use `chrono` for timestamp handling
- UTC timezone consistency
- Accurate timeout calculations

### 3. Cryptography
- Use `sha2` crate for MD5/SHA hashes
- Secure random number generation
- Token generation and validation

### 4. Error Types
- Custom error enum for authentication failures
- Detailed error context
- Proper error propagation

### 5. Async/Await
- Async authentication methods
- Non-blocking token refresh
- Concurrent API request handling

### 6. Storage Patterns
- Use `serde` for token serialization
- Consider encrypted storage libraries
- Secure memory handling

## Example Rust Implementation Outline

```rust
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256, Md5};

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthInfo {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Debug)]
pub struct PixivAuth {
    client_id: String,
    client_secret: String,
    hash_secret: String,
    client: Client,
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl PixivAuth {
    pub async fn auth(&mut self, refresh_token: &str) -> Result<AuthInfo, AuthError> {
        // Generate security headers
        let timestamp = Utc::now().timestamp().to_string();
        let hash_input = format!("{}{}", timestamp, self.hash_secret);
        let hash = format!("{:x}", Md5::digest(hash_input.as_bytes()));

        // Build request
        let mut params = HashMap::new();
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);
        params.insert("grant_type", "refresh_token");
        params.insert("refresh_token", &refresh_token.to_string());

        let mut headers = HeaderMap::new();
        headers.insert("X-Client-Time", timestamp.parse()?);
        headers.insert("X-Client-Hash", hash.parse()?);
        headers.insert("User-Agent", "PixivIOSApp/7.13.3 (iOS 14.6; iPhone13,2)".parse()?);

        // Execute request
        let response = self.client
            .post("https://oauth.secure.pixiv.net/auth/token")
            .form(&params)
            .headers(headers)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AuthError::InvalidCredentials);
        }

        let auth_info: AuthInfo = response.json().await?;

        // Update stored tokens
        self.access_token = Some(auth_info.access_token.clone());
        self.refresh_token = Some(auth_info.refresh_token.clone());
        self.expires_at = Some(Utc::now() + chrono::Duration::seconds(auth_info.expires_in as i64));

        Ok(auth_info)
    }

    pub async fn ensure_auth(&mut self) -> Result<(), AuthError> {
        if let Some(expires_at) = self.expires_at {
            if Utc::now() >= expires_at {
                self.refresh_tokens().await?;
            }
        }
        Ok(())
    }
}
```

## Testing Authentication

### Unit Tests
1. Test header generation
2. Test token parsing
3. Test error scenarios
4. Test token expiration logic

### Integration Tests
1. Test full authentication flow
2. Test token refresh
3. Test invalid credentials
4. Test expired token handling

### Mock Scenarios
- Successful authentication
- Invalid refresh token
- Network errors
- Malformed responses

## Best Practices

1. **Never hardcode tokens** in production code
2. **Implement proper logging** without exposing secrets
3. **Use secure storage** for refresh tokens
4. **Handle edge cases** gracefully
5. **Monitor authentication metrics**
6. **Implement retry logic** with exponential backoff
7. **Validate all inputs** and sanitize responses
8. **Keep dependencies updated** for security patches