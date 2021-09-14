use poem_openapi::{
    auth::{Basic, Bearer},
    registry::{MetaSecurityScheme, Registry},
    OpenApi, SecurityScheme,
};

#[test]
fn rename() {
    #[derive(SecurityScheme)]
    #[oai(name = "ABC", type = "basic")]
    struct MySecurityScheme(Basic);

    assert_eq!(MySecurityScheme::NAME, "ABC");
}

#[test]
fn default_rename() {
    #[derive(SecurityScheme)]
    #[oai(type = "basic")]
    struct MySecurityScheme(Basic);

    assert_eq!(MySecurityScheme::NAME, "my_security_scheme");
}

#[test]
fn desc() {
    /// ABC
    ///
    /// D
    #[derive(SecurityScheme)]
    #[oai(type = "basic")]
    struct MySecurityScheme(Basic);

    let mut registry = Registry::new();
    MySecurityScheme::register(&mut registry);
    assert_eq!(
        registry
            .security_schemes
            .get("MySecurityScheme")
            .unwrap()
            .description,
        Some("ABC\n\nD")
    );
}

#[test]
fn basic_auth() {
    #[derive(SecurityScheme)]
    #[oai(type = "basic")]
    struct MySecurityScheme(Basic);

    let mut registry = Registry::new();
    MySecurityScheme::register(&mut registry);
    assert_eq!(
        registry.security_schemes.get("my_security_scheme").unwrap(),
        &MetaSecurityScheme {
            ty: "http",
            description: None,
            name: None,
            key_in: None,
            scheme: Some("basic"),
            bearer_format: None,
            flows: None,
            openid_connect_url: None
        }
    );

    struct MyApi;

    #[OpenApi]
    impl MyApi {
        #[oai(path = "/test", method = "get")]
        async fn test(&self, #[oai(auth)] auth: MySecurityScheme) {}
    }
}

#[test]
fn bearer_auth() {
    #[derive(SecurityScheme)]
    #[oai(type = "bearer")]
    struct MySecurityScheme(Bearer);

    let mut registry = Registry::new();
    MySecurityScheme::register(&mut registry);
    assert_eq!(
        registry.security_schemes.get("my_security_scheme").unwrap(),
        &MetaSecurityScheme {
            ty: "http",
            description: None,
            name: None,
            key_in: None,
            scheme: Some("bearer"),
            bearer_format: None,
            flows: None,
            openid_connect_url: None
        }
    );
}
