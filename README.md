<h1 align="center">Poem OpenAPI</h1>

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

***

`Poem-openapi` allows you to easily implement APIs that comply with the `OpenAPIv3` specification.
It uses procedural macros to generate a lots of boilerplate code, so that you only need to focus on the more 
important business implementations.

## Features

* **Type safety** If your codes can be compiled, then it is fully compliant with the `OpenAPI v3` specification.
* **Rustfmt friendly** Do not create any DSL that does not conform to Rust's syntax specifications.
* **IDE friendly** Any code generated by the procedural macro will not be used directly.
* **Minimal overhead** All generated code is necessary, and there is almost no overhead.

## Crate features

To avoid compiling unused dependencies, Poem gates certain features, all of which are disabled by default:

|Feature           |Description                     |
|------------------|--------------------------------|
|chrono            | Integrate with the [`chrono` crate](https://crates.io/crates/chrono).          |

## Example

```rust
use poem::listener::TcpListener;
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
    let listener = TcpListener::bind("127.0.0.1:3000");
    poem::Server::new(listener)
        .await
        .unwrap()
        .run(OpenAPI::new(Api).title("hello World").ui_path("/ui"))
        .await
        .unwrap();
}
```

## Run example

Open `http://localhost:3000/ui` in your browser, you will see the `Swagger UI` that contains these API definitions.

```shell
> cargo run --example hello_world

> curl http://localhost:3000
hello!

> curl http://localhost:3000\?name\=sunli
hello, sunli!        
```
