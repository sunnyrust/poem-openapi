use askama::Template;

use crate::poem::{
    endpoint::make_sync,
    route::{get, Route},
    web::Html,
};

const SWAGGER_UI_JS: &'static str = include_str!("swagger-ui-bundle.js");
const SWAGGER_UI_CSS: &'static str = include_str!("swagger-ui.css");

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
<html>
<head>
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="./swagger-ui.css">
    </style>
    <script src="./swagger-ui.js" charset="UTF-8"></script>
</head>
</html>
<body>

<div id="ui"></div>
<script>
    let spec = {{ spec|safe }};

    SwaggerUIBundle({
        dom_id: '#ui',
        spec: spec,
    })
</script>

</body>
"#
)]
struct UITemplate<'a> {
    spec: &'a str,
}

pub fn add_ui_routes(route: Route, mut path: String, document: &str) -> Route {
    if !path.ends_with('/') {
        path.push('/');
    }

    let js_path = format!("{}swagger-ui.js", path);
    let css_path = format!("{}swagger-ui.css", path);

    route
        .at(&js_path, get(make_sync(|_| SWAGGER_UI_JS)))
        .at(&css_path, get(make_sync(|_| SWAGGER_UI_CSS)))
        .at(
            &path,
            get(make_sync({
                let html = UITemplate { spec: &document }.render().unwrap();
                move |_| Html(html.clone())
            })),
        )
}
