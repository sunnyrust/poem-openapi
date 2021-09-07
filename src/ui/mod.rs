use askama::Template;

use crate::poem::{
    endpoint::make_sync,
    route::{get, Route},
    web::Html,
};

const SWAGGER_UI_JS: &str = include_str!("swagger-ui-bundle.js");
const SWAGGER_UI_CSS: &str = include_str!("swagger-ui.css");

#[derive(Template)]
#[template(
    ext = "html",
    source = r#"
<html charset="UTF-8">
<head>
    <meta http-equiv="Content-Type" content="text/html;charset=utf-8">
    <title>Swagger UI</title>
    <style charset="UTF-8">{{ css|safe }}</style>
    <script charset="UTF-8">{{ script|safe }}</script>
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
    script: &'static str,
    css: &'static str,
}

pub fn add_ui_routes(route: Route, path: String, document: &str) -> Route {
    route.at(
        &path,
        get(make_sync({
            let html = UITemplate {
                spec: document,
                script: SWAGGER_UI_JS,
                css: SWAGGER_UI_CSS,
            }
            .render()
            .unwrap();
            move |_| Html(html.clone())
        })),
    )
}
