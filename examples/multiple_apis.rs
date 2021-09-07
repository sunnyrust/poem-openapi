use poem_openapi::{OpenAPI, API};

struct Api1;

#[API]
impl Api1 {
    #[oai(path = "/a", method = "get")]
    async fn test(&self) {}
}

struct Api2;

#[API]
impl Api2 {
    #[oai(path = "/b", method = "post")]
    async fn test1(&self) {}

    #[oai(path = "/b", method = "get")]
    async fn test2(&self) {}
}

struct Api3;

#[API]
impl Api3 {
    #[oai(path = "/c", method = "post")]
    async fn test1(&self) {}

    #[oai(path = "/c", method = "get")]
    async fn test2(&self) {}
}

#[tokio::main]
async fn main() {
    poem::Server::bind("127.0.0.1:3000")
        .await
        .unwrap()
        .run(
            OpenAPI::new(Api1.combine(Api2).combine(Api3))
                .title("hello World")
                .ui_path("/"),
        )
        .await
        .unwrap();
}
