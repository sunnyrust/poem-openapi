use poem::{route, route::Route, IntoEndpoint};

use crate::{
    registry::{Document, MetaInfo, MetaServer, MetaTag, Registry},
    ui::add_ui_routes,
    API,
};

/// An OpenAPI container for Poem.
pub struct OpenAPI<T> {
    api: T,
    info: Option<MetaInfo>,
    servers: Vec<MetaServer>,
    tags: Vec<MetaTag>,
    ui_path: Option<String>,
}

impl<T> OpenAPI<T> {
    /// Create an OpenAPI container.
    #[must_use]
    pub fn new(api: T) -> Self {
        Self {
            api,
            info: None,
            servers: Vec::new(),
            tags: Vec::new(),
            ui_path: None,
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

    /// Appends a tag to the API container.
    ///
    /// Reference: <https://github.com/OAI/OpenAPI-Specification/blob/main/versions/3.1.0.md#tagObject>
    #[must_use]
    pub fn tag(mut self, name: &'static str) -> Self {
        self.tags.push(MetaTag {
            name,
            description: None,
        });
        self
    }

    /// Appends a tag and description to the API container.
    #[must_use]
    pub fn tag_with_description(mut self, name: &'static str, description: &'static str) -> Self {
        self.tags.push(MetaTag {
            name,
            description: Some(description),
        });
        self
    }

    /// Sets the URL path to access Swagger UI.
    ///
    /// NOTE: You must set this path before the API container will create a
    /// route to Swagger UI.
    #[must_use]
    pub fn ui_path(self, path: impl Into<String>) -> Self {
        Self {
            ui_path: Some(path.into()),
            ..self
        }
    }
}

impl<T: API> IntoEndpoint for OpenAPI<T> {
    type Endpoint = Route;

    fn into_endpoint(self) -> Self::Endpoint {
        let mut route = self.api.add_routes(route());

        if let Some(ui_path) = self.ui_path {
            let mut registry = Registry::new();
            let metadata = T::metadata();

            T::register(&mut registry);

            let doc = Document {
                info: self.info.as_ref(),
                servers: &self.servers,
                apis: &metadata,
                tags: &self.tags,
                registry: &registry,
            };
            let doc_json = serde_json::to_string_pretty(&doc).unwrap();
            route = add_ui_routes(route, ui_path, &doc_json);
        }

        route
    }
}
