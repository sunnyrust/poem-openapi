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
        .run(OpenAPI::new(Api).title("hello World").ui_path("/ui"))
        .await
        .unwrap();
}
