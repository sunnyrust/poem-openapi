use indexmap::IndexMap;
use serde::{
    ser::{SerializeMap, SerializeStruct},
    Serialize, Serializer,
};

use crate::registry::{
    MetaAPI, MetaInfo, MetaPath, MetaResponses, MetaSchema, MetaSchemaRef, MetaServer, MetaTag,
    Registry,
};

const OPENAPI_VERSION: &str = "3.0.0";

struct PathMap<'a>(&'a [MetaAPI]);

impl<'a> Serialize for PathMap<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_map(Some(self.0.len()))?;
        for api in self.0 {
            for path in api.paths {
                s.serialize_entry(path.path, path)?;
            }
        }
        s.end()
    }
}

impl Serialize for MetaPath {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_map(None)?;

        for operation in self.operations {
            s.serialize_entry(&operation.method.to_string().to_lowercase(), operation)?;
        }

        s.end()
    }
}

impl Serialize for MetaSchemaRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MetaSchemaRef::Inline(data_type) => {
                let mut s = serializer.serialize_struct("Schema", 2)?;
                s.serialize_field("type", data_type.ty)?;
                s.serialize_field("format", &data_type.format)?;
                s.end()
            }
            MetaSchemaRef::Reference(name) => {
                let mut s = serializer.serialize_struct("Schema", 1)?;
                s.serialize_field("$ref", &format!("#/components/schemas/{}", name))?;
                s.end()
            }
        }
    }
}

impl Serialize for MetaResponses {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_map(None)?;
        for resp in self.responses {
            match resp.status {
                Some(status) => s.serialize_entry(&format!("{}", status), resp)?,
                None => s.serialize_entry("default", resp)?,
            }
        }
        s.end()
    }
}

pub(crate) struct Document<'a> {
    pub(crate) info: Option<&'a MetaInfo>,
    pub(crate) servers: &'a [MetaServer],
    pub(crate) apis: &'a [MetaAPI],
    pub(crate) tags: &'a [MetaTag],
    pub(crate) registry: &'a Registry,
}

impl<'a> Serialize for Document<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Components<'a> {
            schemas: &'a IndexMap<&'static str, MetaSchema>,
        }

        let mut s = serializer.serialize_struct("OpenAPI", 6)?;

        s.serialize_field("openapi", OPENAPI_VERSION)?;
        s.serialize_field("info", &self.info)?;
        s.serialize_field("servers", self.servers)?;
        s.serialize_field("tags", self.tags)?;
        s.serialize_field("paths", &PathMap(&self.apis))?;
        s.serialize_field(
            "components",
            &Components {
                schemas: &self.registry.schemas,
            },
        )?;

        s.end()
    }
}
