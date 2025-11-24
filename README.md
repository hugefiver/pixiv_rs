# Pixiv API Rust Client (Pixiv-RS)

这是一个将 Python 库 [PixivPy](https://github.com/upbit/pixivpy) 移植到 Rust 的项目。它提供了一个类型安全、异步的客户端库，用于与 Pixiv API 进行交互。

**注意**：本项目是 [PixivPy](https://github.com/upbit/pixivpy) 的 Rust 移植版本。我们致力于保持 API 接口和行为与上游项目一致。上游项目代码作为 submodule 保存在 `docs/pixivpy/` 目录下，**请勿修改其中的任何文件**。

## 特性

- **OAuth2 认证**：支持使用 refresh token 进行认证。
- **API 覆盖**：提供对 Pixiv App API 和 Public API 的访问。
- **SNI 绕过**：支持通过 IP 直连绕过部分网络限制。
- **异步运行时**：基于 `tokio` 构建，支持高性能并发。
- **类型安全**：使用 `serde` 进行数据模型序列化，提供编译时类型检查。
- **详细日志**：集成 `tracing`，方便调试和监控。
- **Vibe Coding**：提供 `Claude.md` 作为 AI 辅助开发的上下文规范。

## 快速开始

### 安装

将以下内容添加到你的 `Cargo.toml` 文件中：

```toml
[dependencies]
pixiv_rs = { git = "https://github.com/hugefiver/pixiv_rs", branch = "master" }
tokio = { version = "1", features = ["full"] }
```

### 基本用法

```rust
use pixiv_rs::{AuthClient, AppClient};
use pixiv_rs::network::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pixiv_rs::init_logging();

    let mut auth_client = AuthClient::new()?;
    let auth_response = auth_client.login_with_token("YOUR_REFRESH_TOKEN").await?;
    println!("登录成功! 用户 ID: {}", auth_response.user.id);

    let http_client = HttpClient::new()?;
    http_client.set_access_token(auth_response.access_token.clone());
    let app_client = AppClient::new(http_client);

    let illust_detail = app_client.illust_detail(12345678).await?;
    println!("插画标题: {}", illust_detail.illust.title);

    Ok(())
}
```

### SNI 绕过用法

```rust
use pixiv_rs::client::bypass_sni::BypassSniAppClient;
use pixiv_rs::network::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_client = BypassSniAppClient::with_ip("210.140.131.145")?; // app-api.pixiv.net
    
    app_client.http_client.set_access_token("YOUR_ACCESS_TOKEN".to_string());
    
    let illust_detail = app_client.illust_detail(12345678).await?;
    println!("插画标题: {}", illust_detail.illust.title);

    Ok(())
}
```

## 开发指南

### 构建项目

```bash
git clone --recurse-submodules https://github.com/hugefiver/pixiv_rs.git
cd pixiv_rs

cargo build
```

### 运行测试

```bash
cargo test

cargo test auth
```

### 代码规范与 Vibe Coding

本项目遵循一套严格的开发规范，以确保代码质量和与上游的一致性。核心规范文档为 `Claude.md`。

- **核心原则**：**绝对不要修改 `docs/pixivpy` 目录中的任何文件**。
- **Vibe Coding**：在进行任何开发工作前，请务必阅读 `Claude.md`。它定义了项目的架构映射、代码风格、错误处理方式等关键信息，是 AI 辅助编程和人工开发的重要上下文。

## 项目结构

```
pixiv_rs/
├── Cargo.toml              # 项目配置文件
├── src/                    # Rust 源代码
│   ├── lib.rs              # 库入口
│   ├── auth.rs             # 认证模块
│   ├── client/             # API 客户端模块
│   │   ├── app.rs          # App API 客户端
│   │   ├── public.rs       # Public API 客户端
│   │   └── bypass_sni.rs   # SNI 绕过客户端
│   ├── models/             # 数据模型
│   │   ├── app.rs          # App API 数据模型
│   │   └── public.rs       # Public API 数据模型
│   ├── network/            # 网络层
│   ├── error.rs            # 错误类型定义
│   └── utils.rs            # 工具函数
├── tests/                  # 集成测试
├── examples/               # 使用示例
├── docs/                   # 文档
│   └── pixivpy/            # 上游 PixivPy 项目 (submodule)
│       └── pixivpy/        # 上游源码 (请勿修改)
├── README.md               # 本文件
├── Claude.md               # Vibe Coding 核心规范
└── LICENSE                 # 许可证
```

## 许可证

本项目根据 MIT 许可证授权。有关详细信息，请参阅 [LICENSE](LICENSE) 文件。
