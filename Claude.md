# PixivPy Rust 移植项目

## 核心原则

**绝对不要修改 `docs/pixivpy` 目录中的任何文件。** 这些文件是上游 Python 项目 PixivPy 的原始代码，作为我们移植工作的参考基准。所有开发工作都应在 `src/` 目录下进行。

## 项目目标

本项目旨在将 Python 库 [PixivPy](https://github.com/upbit/pixivpy) 完整地移植到 Rust 语言。目标是创建一个功能等价、类型安全、高性能的 Rust 客户端库，用于与 Pixiv API 交互。

- **API 兼容性**：Rust 实现必须与上游 PixivPy 的 API 接口（方法名、参数、返回类型）保持一致。
- **行为一致性**：在相同输入下，Rust 版本应产生与 Python 版本相同或极其相似的输出和副作用（如网络请求、错误处理）。
- **代码质量**：遵循 Rust 社区的最佳实践，利用 Rust 的所有权、借用检查和类型系统优势。

## 代码规范与风格指南

### 1. 错误处理

- 使用 [`thiserror`](https://crates.io/crates/thiserror) crate 来定义自定义错误类型。
- 为不同的错误场景定义明确的错误变体（如 `NetworkError`, `AuthError`, `ApiError`）。
- 使用 `Result<T, PixivError>` 作为函数返回类型。
- 参考 `src/error.rs` 中的 `PixivError` 定义。

### 2. 异步编程

- 所有涉及 I/O 的操作（网络请求）必须是异步的。
- 使用 `async/await` 语法。
- 依赖 [`tokio`](https://crates.io/crates/tokio) 作为异步运行时。
- 参考 `src/client/app.rs` 中的异步方法实现。

### 3. HTTP 客户端

- 使用 [`reqwest`](https://crates.io/crates/reqwest) crate 进行 HTTP 请求。
- 封装一个 `HttpClient` 结构体来管理共享的客户端配置和状态（如默认 headers, access token）。
- 参考 `src/network/mod.rs` 和 `src/client/app.rs` 中的 `HttpClient` 使用方式。

### 4. 数据模型

- 使用 [`serde`](https://crates.io/crates/serde) 进行 JSON 序列化和反序列化。
- 使用 `#[derive(Serialize, Deserialize)]` 宏。
- 字段命名遵循 `snake_case`，并通过 `#[serde(rename = "camelCase")]` 与 API 的 `camelCase` 字段名映射。
- 对于枚举类型，实现 `ToString` trait 以方便转换为 API 所需的字符串参数。
- 参考 `src/models/app.rs` 中的数据结构定义。

### 5. 模块结构

- 遵循 `src/` 目录下的现有结构：
  - `auth/`: 认证相关逻辑。
  - `client/`: 不同类型的 API 客户端（App API, Public API, Bypass SNI）。
  - `models/`: 数据模型定义。
  - `network/`: 网络层抽象。
  - `utils/`: 通用工具函数。
  - `error.rs`: 全局错误类型定义。
  - `lib.rs`: 库的入口和公共 API 导出。

### 6. 文档注释

- 为所有公共的 structs, enums, functions, methods 提供 Rustdoc 注释。
- 使用 `///` 格式。
- 包含简要描述、参数说明（`# Arguments`）、返回值说明（`# Returns`）和使用示例（`# Example`）。
- 参考 `src/client/app.rs` 中的文档风格。

## 架构映射 (Python to Rust)

参考 `docs/pixivpy/10_rust_migration_guide.md` 中的映射关系：

| Python 模块 (PixivPy) | Rust 模块 (Pixiv-RS) |
| :--- | :--- |
| `pixivpy3.api.BasePixivAPI` | `src/network/HttpClient` + `src/auth/` |
| `pixivpy3.aapi.AppPixivAPI` | `src/client/app.rs` (`AppClient`) |
| `pixivpy3.bapi.ByPassSniApi` | `src/client/bypass_sni.rs` (`BypassSniAppClient`) |
| `pixivpy3.models` | `src/models/` |
| `pixivpy3.utils` | `src/utils.rs` |

## 开发流程

1. **参考上游**：在实现任何功能前，仔细阅读 `docs/pixivpy/` 下对应的 Python 代码和文档。
2. **编写代码**：在 `src/` 目录下实现功能，严格遵循上述规范。
3. **编写测试**：为新功能添加单元测试或集成测试，放在 `tests/` 目录下。
4. **文档完善**：更新相关模块的 Rustdoc 注释。
5. **代码审查**：确保代码符合规范，逻辑正确。

## Vibe Coding 指南

当你需要为本项目编写代码时，请始终将此文档作为你的核心上下文。它定义了项目的基调、规则和期望。在编码时，请时刻思考：

- 我的代码是否遵循了 Rust 的最佳实践？
- 我的错误处理是否清晰且符合项目规范？
- 我的 API 设计是否与上游 PixivPy 保持一致？
- 我有没有意外修改 `docs/pixivpy` 中的文件？
- 我的代码是否添加了适当的文档注释？

通过遵循这些原则，我们可以确保项目的一致性和高质量。
