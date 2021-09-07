<h1 align="center"><code>Poem OpenAPI</code></h1>

<p align="center">Fast and Type-Safe OpenAPI implementation for Poem.</p>
<div align="center">
  <!-- CI -->
  <img src="https://github.com/poem-web/poem-openapi/workflows/CI/badge.svg" />
  <!-- codecov -->
  <img src="https://codecov.io/gh/poem-web/poem-openapi/branch/master/graph/badge.svg" />
  <!-- Crates version -->
  <a href="https://crates.io/crates/poem-openapi">
    <img src="https://img.shields.io/crates/v/poem-openapi.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/poem-openapi">
    <img src="https://img.shields.io/crates/d/poem-openapi.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs.rs docs -->
  <a href="https://docs.rs/poem-openapi">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <a href="https://github.com/rust-secure-code/safety-dance/">
    <img src="https://img.shields.io/badge/unsafe-forbidden-success.svg?style=flat-square"
      alt="Unsafe Rust forbidden" />
  </a>
  <a href="https://blog.rust-lang.org/2021/07/29/Rust-1.54.0.html">
    <img src="https://img.shields.io/badge/rustc-1.54+-ab6000.svg"
      alt="rustc 1.54+" />
  </a>
</div>

## Example

```rust
use poem_openapi::{payload::PlainText, OpenAPI, API};

struct Api;

#[API]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index(&self, #[oai(name = "name", in = "query")] name: Option<String>) -> PlainText {
        match name {
            Some(name) => format!("hello, {}!", name).into(),
            None => "hello!".into(),
        }
    }
}

#[tokio::main]
async fn main() {
    poem::Server::bind("127.0.0.1:3000")
        .await
        .unwrap()
        .run(OpenAPI::new(Api).title("?hello World"))
        .await
        .unwrap();
}
```

```
> curl http://localhost:3000
hello!

> curl http://localhost:3000\?name\=sunli
hello, sunli!                      
```