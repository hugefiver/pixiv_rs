# Network Bypass Strategy

## Overview

PixivPy provides sophisticated network bypass capabilities to handle various restrictions including DNS blocking, ISP filtering, and Cloudflare protection. This document details the network bypass implementation using DNS over HTTPS (DoH) and SNI bypass techniques.

## BypassSniApi Architecture

### 1. Overview

The `ByPassSniApi` class extends `BasePixivAPI` with network bypass capabilities:
- DNS over HTTPS (DoH) for bypassing DNS filtering
- Host header SSL adapter for SNI bypass
- Fallback to multiple DNS providers
- IP-based connection when domain resolution fails

### 2. Class Structure

```python
from pixivpy3 import AppPixivAPI
from requests_toolbelt.adapters.host_header_ssl import HostHeaderSSLAdapter

class ByPassSniApi(AppPixivAPI):
    def __init__(self, **kwargs):
        self.hosts = {}  # Maps hostname -> IP
        self.require_appapi_hosts()
        super().__init__(**kwargs)

    def require_appapi_hosts(self):
        """Resolve real IP addresses for Pixiv hosts"""
        # Primary Pixiv API host
        self.hosts["app-api.pixiv.net"] = self._resolve_host("app-api.pixiv.net")

        # OAuth host
        self.hosts["oauth.secure.pixiv.net"] = self._resolve_host("oauth.secure.pixiv.net")

        # Image hosts
        self.hosts["i.pximg.net"] = self._resolve_host("i.pximg.net")
        self.hosts["s.pximg.net"] = self._resolve_host("s.pximg.net")
```

## DNS over HTTPS Implementation

### 1. Multiple DoH Providers

```python
import json
import urllib.parse

class DnsResolver:
    def __init__(self):
        self.providers = [
            "https://cloudflare-dns.com/dns-query",
            "https://1.1.1.1/dns-query",
            "https://dns.google/resolve",
            "https://doh.opendns.com/dns-query"
        ]

    def resolve_via_doh(self, domain, record_type="A"):
        """Resolve domain using DNS over HTTPS"""

        params = {
            "name": domain,
            "type": record_type
        }

        headers = {
            "Accept": "application/dns-json"
        }

        for provider in self.providers:
            try:
                # Cloudflare and Google use different formats
                if "cloudflare" in provider or "1.1.1.1" in provider:
                    # RFC 8484 format
                    response = self.session.get(
                        f"{provider}?{urllib.parse.urlencode(params)}",
                        headers=headers
                    )
                else:
                    # Google DNS API format
                    response = self.session.get(
                        provider,
                        params=params,
                        headers=headers
                    )

                if response.status_code == 200:
                    data = response.json()

                    # Parse different response formats
                    if "Answer" in data:
                        # Google DNS format
                        for answer in data["Answer"]:
                            if answer.get("type") == 1:  # A record
                                return answer["data"]
                    elif "Answer" not in data:
                        # Cloudflare format
                        for answer in data.get("Answer", []):
                            if answer.get("type") == 1:
                                return answer["data"]

            except Exception as e:
                print(f"Provider {provider} failed: {e}")
                continue

        raise PixivError(f"Failed to resolve {domain} via DoH")
```

### 2. Fallback to Traditional DNS

```python
import socket
import dns.resolver

class DnsResolver:
    def resolve_traditional(self, domain):
        """Traditional DNS resolution as fallback"""
        try:
            # Try system DNS first
            ip = socket.gethostbyname(domain)
            return ip
        except:
            pass

        try:
            # Try with dnspython for better control
            resolver = dns.resolver.Resolver()
            resolver.nameservers = [
                "8.8.8.8",      # Google
                "1.1.1.1",      # Cloudflare
                "208.67.222.222"  # OpenDNS
            ]

            result = resolver.resolve(domain, "A")
            return result[0].address
        except:
            pass

        raise PixivError(f"Failed to resolve {domain}")
```

## Host Header SSL Adapter

### 1. SSL Adapter Implementation

```python
from requests.adapters import HTTPAdapter
from urllib3.util.ssl_ import create_urllib3_context
from requests_toolbelt.adapters.host_header_ssl import HostHeaderSSLAdapter

class PixivSSLAdapter(HostHeaderSSLAdapter):
    """Custom SSL adapter for SNI bypass"""

    def init_poolmanager(self, *args, **kwargs):
        """Initialize pool manager with custom SSL context"""

        # Create custom SSL context
        context = create_urllib3_context()

        # Disable SNI verification
        context.check_hostname = False

        # Allow weaker ciphers if needed
        context.set_ciphers("DEFAULT:@SECLEVEL=1")

        # Disable verification for bypass scenarios
        context.verify_mode = ssl.CERT_NONE

        kwargs["ssl_context"] = context
        kwargs["assert_hostname"] = False

        return super().init_poolmanager(*args, **kwargs)
```

### 2. Mounting the Adapter

```python
def _setup_bypass_adapter(self, hostname, ip):
    """Mount custom adapter for specific host"""

    # Create adapter with IP address
    adapter = self.PixivSSLAdapter()

    # Mount for HTTPS to IP address
    self.session.mount(
        f"https://{ip}/",
        adapter
    )

    # Store mapping
    self.ip_mappings[hostname] = ip
```

## Request Bypass Flow

### 1. Modified Request Method

```python
def requests_call_with_bypass(self, method, url, **kwargs):
    """Make request with SNI bypass if needed"""

    from urllib.parse import urlparse

    parsed_url = urlparse(url)
    hostname = parsed_url.hostname

    # Check if we have a bypass IP for this host
    if hostname in self.hosts and self.hosts[hostname] != hostname:
        ip = self.hosts[hostname]

        # Replace hostname with IP in URL
        bypass_url = url.replace(hostname, ip)

        # Add Host header
        headers = kwargs.get('headers', {})
        headers['Host'] = hostname
        kwargs['headers'] = headers

        # Make request to IP with original hostname in Host header
        return super().requests_call(method, bypass_url, **kwargs)

    # Fallback to normal request
    return super().requests_call(method, url, **kwargs)
```

### 2. Automatic Retry Logic

```python
def _request_with_retry(self, method, url, **kwargs):
    """Make request with automatic fallback"""

    try:
        # Try normal request first
        return self.requests_call(method, url, **kwargs)

    except (requests.exceptions.ConnectionError,
            requests.exceptions.SSLError,
            PixivError) as e:

        # Check if this is a DNS/blocking error
        if self._is_blocking_error(e):
            parsed_url = urlparse(url)
            hostname = parsed_url.hostname

            # Try to resolve fresh IP
            if hostname:
                try:
                    new_ip = self._resolve_host(hostname)
                    self.hosts[hostname] = new_ip

                    # Retry with bypass
                    return self.requests_call_with_bypass(
                        method, url, **kwargs
                    )
                except:
                    pass

        # Re-raise original error
        raise

def _is_blocking_error(self, error):
    """Check if error is likely due to blocking"""

    error_str = str(error).lower()
    blocking_indicators = [
        "name resolution failed",
        "nodename nor servname provided",
        "temporary failure in name resolution",
        "ssl: wrong_version_number",
        "certificate verify failed"
    ]

    return any(indicator in error_str for indicator in blocking_indicators)
```

## Host Resolution Strategy

### 1. Caching Mechanism

```python
import time
from pathlib import Path

class HostCache:
    def __init__(self, cache_file="host_cache.json"):
        self.cache_file = Path(cache_file)
        self.cache = {}
        self.ttl = 3600  # 1 hour TTL
        self._load_cache()

    def _load_cache(self):
        """Load cached IP addresses"""
        if self.cache_file.exists():
            try:
                data = json.loads(self.cache_file.read_text())
                now = time.time()

                # Filter expired entries
                self.cache = {
                    host: info for host, info in data.items()
                    if now - info.get("timestamp", 0) < self.ttl
                }
            except:
                self.cache = {}

    def _save_cache(self):
        """Save IP addresses to cache"""
        self.cache_file.write_text(json.dumps(self.cache, indent=2))

    def get(self, host):
        """Get cached IP if valid"""
        if host in self.cache:
            info = self.cache[host]
            if time.time() - info["timestamp"] < self.ttl:
                return info["ip"]
        return None

    def set(self, host, ip):
        """Cache IP address"""
        self.cache[host] = {
            "ip": ip,
            "timestamp": time.time()
        }
        self._save_cache()
```

### 2. Resolution with Caching

```python
class ByPassSniApi:
    def __init__(self, **kwargs):
        self.host_cache = HostCache()
        self.hosts = {}
        self.require_appapi_hosts()
        super().__init__(**kwargs)

    def _resolve_host(self, hostname):
        """Resolve host with caching"""

        # Check cache first
        cached_ip = self.host_cache.get(hostname)
        if cached_ip:
            return cached_ip

        # Resolve with DoH
        try:
            ip = self._resolve_via_doh(hostname)
        except:
            # Fallback to traditional DNS
            ip = self._resolve_traditional(hostname)

        # Cache the result
        self.host_cache.set(hostname, ip)

        return ip
```

## Special Considerations

### 1. Image Download Bypass

```python
def download_with_bypass(self, url, **kwargs):
    """Download image with bypass if needed"""

    from urllib.parse import urlparse
    parsed_url = urlparse(url)
    hostname = parsed_url.hostname

    # Special handling for Pixiv image hosts
    if hostname in ["i.pximg.net", "s.pximg.net"]:
        if hostname in self.hosts:
            # Use IP bypass
            ip = self.hosts[hostname]

            # Update URL
            bypass_url = url.replace(hostname, ip)

            # Add proper referer and host headers
            headers = kwargs.get('headers', {})
            headers.update({
                'Host': hostname,
                'Referer': 'https://app-api.pixiv.net/'
            })
            kwargs['headers'] = headers

            return self.download(bypass_url, **kwargs)

    # Standard download
    return super().download(url, **kwargs)
```

### 2. WebSocket Support (if needed)

```python
import websocket

class PixivWebSocket:
    def __init__(self, host, ip):
        self.host = host
        self.ip = ip

    def connect(self):
        """Connect via WebSocket with host header"""

        # Custom headers for SNI bypass
        headers = {
            "Host": self.host
        }

        # Connect to IP with host header
        ws = websocket.WebSocket(
            f"wss://{self.ip}/ws",
            header=headers,
            sslopt={"cert_reqs": ssl.CERT_NONE}
        )

        return ws
```

## Error Detection and Recovery

### 1. Connectivity Test

```python
def test_connectivity(self, hostname, ip):
    """Test if we can connect via IP bypass"""

    import socket
    import ssl

    context = ssl.create_default_context()
    context.check_hostname = False
    context.verify_mode = ssl.CERT_NONE

    try:
        with socket.create_connection((ip, 443), timeout=5) as sock:
            with context.wrap_socket(sock, server_hostname=hostname) as ssock:
                # Send HTTP request
                request = f"GET / HTTP/1.1\r\nHost: {hostname}\r\n\r\n"
                ssock.send(request.encode())

                # Read response
                response = ssock.recv(1024)

                # Check if we got a valid HTTP response
                return response.startswith(b"HTTP/")
    except:
        return False
```

### 2. Automatic IP Rotation

```python
def get_working_ip(self, hostname):
    """Get a working IP for hostname"""

    # Try multiple IPs from different providers
    for _ in range(3):
        try:
            ip = self._resolve_host(hostname)

            # Test connectivity
            if self.test_connectivity(hostname, ip):
                self.hosts[hostname] = ip
                return ip
        except:
            continue

    raise PixivError(f"No working IP found for {hostname}")
```

## Rust Implementation Strategy

### 1. DoH Client Implementation

```rust
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct CloudflareResponse {
    #[serde(rename = "Answer")]
    answer: Option<Vec<DnsAnswer>>,
}

#[derive(Debug, Deserialize)]
struct DnsAnswer {
    #[serde(rename = "type")]
    record_type: u16,
    data: String,
}

pub struct DohResolver {
    client: Client,
    providers: Vec<String>,
}

impl DohResolver {
    pub async fn resolve(&self, domain: &str) -> Result<String, PixivError> {
        for provider in &self.providers {
            match self.try_provider(provider, domain).await {
                Ok(ip) => return Ok(ip),
                Err(_) => continue,
            }
        }
        Err(PixivError::DnsResolutionFailed(domain.to_string()))
    }

    async fn try_provider(&self, provider: &str, domain: &str) -> Result<String, PixivError> {
        let url = format!(
            "{}?name={}&type=A",
            provider,
            urlencoding::encode(domain)
        );

        let response = self.client
            .get(&url)
            .header("Accept", "application/dns-json")
            .send()
            .await?;

        if response.status().is_success() {
            let data: CloudflareResponse = response.json().await?;

            if let Some(answers) = data.answer {
                for answer in answers {
                    if answer.record_type == 1 {  // A record
                        return Ok(answer.data);
                    }
                }
            }
        }

        Err(PixivError::InvalidResponse)
    }
}
```

### 2. SNI Bypass Implementation

```rust
use reqwest::{Client, RequestBuilder, Url};
use std::sync::Arc;

#[derive(Clone)]
pub struct BypassClient {
    client: Client,
    host_mappings: Arc<DashMap<String, String>>,
    resolver: DohResolver,
}

impl BypassClient {
    pub async fn request_with_bypass(
        &self,
        method: reqwest::Method,
        url: &str,
    ) -> Result<reqwest::Response, PixivError> {
        let url = Url::parse(url)?;
        let hostname = url.host_str().ok_or(PixivError::InvalidUrl)?;

        // Check if we have bypass IP
        let bypass_url = if let Some(ip) = self.host_mappings.get(hostname) {
            url.host_str().map(|_| {
                url.set_host_ip(Some(ip.parse().unwrap()));
                url
            })?;
            url
        } else {
            url
        };

        // Build request
        let mut request = self.client.request(method.clone(), bypass_url);

        // Add Host header if using bypass
        if hostname != bypass_url.host_str().unwrap_or("") {
            request = request.header("Host", hostname);
        }

        // Execute with retry logic
        self.execute_with_retry(request, hostname).await
    }

    async fn execute_with_retry(
        &self,
        mut request: RequestBuilder,
        hostname: &str,
    ) -> Result<reqwest::Response, PixivError> {
        // Try normal request first
        let result = request.try_clone().unwrap().send().await;

        match result {
            Ok(response) => Ok(response),
            Err(e) if self.is_blocking_error(&e) => {
                // Try to resolve new IP
                let ip = self.resolver.resolve(hostname).await?;
                self.host_mappings.insert(hostname.to_string(), ip.clone());

                // Update URL with new IP
                let mut url = request.url().clone();
                url.set_host_ip(Some(ip.parse().unwrap()));

                // Update Host header
                request = request.header("Host", hostname);

                // Retry with new IP
                request.send().await.map_err(PixivError::from)
            }
            Err(e) => Err(PixivError::from(e)),
        }
    }
}
```

### 3. TLS Configuration for Bypass

```rust
use rustls::{ClientConfig, RootCertStore, ServerName};
use rustls_native_certs::load_native_certs;
use webpki_roots;

pub fn create_bypass_tls_config() -> ClientConfig {
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_custom_certificate_verifier(Arc::new(InsecureVerifier {}))
        .with_no_client_auth();

    // Allow any server name (disable SNI verification)
    config.enable_sni = false;

    // Use weak ciphers if needed
    config.ciphersuites = rustls::ALL_CIPHERSUITES;

    config
}

struct InsecureVerifier;

impl rustls::ServerCertVerifier for InsecureVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &rustls::Certificate,
        _intermediates: &[rustls::Certificate],
        _server_name: &rustls::ServerName,
        _scts: &mut dyn Iterator<Item = &[u8]>,
        _ocsp_response: &[u8],
        _now: std::time::SystemTime,
    ) -> Result<rustls::ClientCertVerified, rustls::TLSError> {
        Ok(rustls::ClientCertVerified::assertion())
    }
}
```

### 4. Connection Pool with Bypass

```rust
use tokio::sync::RwLock;

pub struct BypassConnectionPool {
    normal_client: Client,
    bypass_client: BypassClient,
    bypass_hosts: RwLock<HashSet<String>>,
}

impl BypassConnectionPool {
    pub async fn get_client_for_host(&self, host: &str) -> &dyn HttpClient {
        let bypass_hosts = self.bypass_hosts.read().await;

        if bypass_hosts.contains(host) {
            &self.bypass_client
        } else {
            &self.normal_client
        }
    }

    pub async fn mark_host_bypass(&self, host: &str) {
        let mut bypass_hosts = self.bypass_hosts.write().await;
        bypass_hosts.insert(host.to_string());
    }
}
```

## Best Practices

1. **Always implement fallbacks** - Don't rely solely on bypass
2. **Cache IP addresses** with appropriate TTL
3. **Test connectivity** before using resolved IPs
4. **Handle errors gracefully** and provide meaningful messages
5. **Monitor for IP changes** and update cache accordingly
6. **Use multiple DNS providers** for reliability
7. **Implement proper timeout handling** for DNS resolution
8. **Log bypass usage** for debugging
9. **Consider user configuration** - Some networks don't need bypass
10. **Be aware of legal implications** in different jurisdictions