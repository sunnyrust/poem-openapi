use poem::{route, route::Route, IntoEndpoint};

use crate::{
    poem::Endpoint,
    registry::{Document, MetaInfo, MetaServer, Registry},
    ui::create_ui_endpoint,
    OpenApi,
};

/// An OpenAPI service for Poem.
pub struct OpenApiService<T> {
    api: T,
    info: Option<MetaInfo>,
    servers: Vec<MetaServer>,
}

impl<T> OpenApiService<T> {
    /// Create an OpenAPI container.
    #[must_use]
    pub fn new(api: T) -> Self {
        Self {
            api,
            info: None,
            servers: Vec::new(),
        }
    }

    /// Sets the title of the API container.
    ///
    /// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#infoObject>
    #[must_use]
    pub fn title(mut self, title: &'static str) -> Self {
        self.info.get_or_insert_with(Default::default).title = Some(title);
        self
    }

    /// Sets the description of the API container.
    #[must_use]
    pub fn description(mut self, description: &'static str) -> Self {
        self.info.get_or_insert_with(Default::default).description = Some(description);
        self
    }

    /// Sets the version of the API container.
    ///
    /// NOTE: The version of the OpenAPI document (which is distinct from the
    /// OpenAPI Specification version or the API implementation version).
    ///
    /// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#infoObject>
    #[must_use]
    pub fn version(mut self, version: &'static str) -> Self {
        self.info.get_or_insert_with(Default::default).version = Some(version);
        self
    }

    /// Appends a server to the API container.
    ///
    /// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#serverObject>
    #[must_use]
    pub fn server(mut self, url: &'static str) -> Self {
        self.servers.push(MetaServer {
            url,
            description: None,
        });
        self
    }

    /// Appends a server and description to the API container.
    #[must_use]
    pub fn server_with_description(mut self, url: &'static str, description: &'static str) -> Self {
        self.servers.push(MetaServer {
            url,
            description: Some(description),
        });
        self
    }

    /// Create the Swagger UI endpoint.
    #[must_use]
    pub fn swagger_ui(&self, absolute_uri: impl AsRef<str>) -> impl Endpoint
    where
        T: OpenApi,
    {
        let mut registry = Registry::new();
        let metadata = T::meta();

        T::register(&mut registry);

        let doc = Document {
            info: self.info.as_ref(),
            servers: &self.servers,
            apis: &metadata,
            registry: &registry,
        };
        let doc_json = serde_json::to_string_pretty(&doc).unwrap();
        create_ui_endpoint(absolute_uri.as_ref(), &doc_json)
    }
}

impl<T: OpenApi> IntoEndpoint for OpenApiService<T> {
    type Endpoint = Route;

    fn into_endpoint(self) -> Self::Endpoint {
        self.api.add_routes(route())
    }
}
