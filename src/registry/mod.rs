mod ser;

use indexmap::{IndexMap, IndexSet};
use poem::http::Method;
pub(crate) use ser::Document;
use serde::Serialize;

use crate::{
    base::Schema,
    serde::{ser::SerializeMap, Serializer},
    serde_json::Value,
    types::DataType,
};

#[derive(Debug, Default, Eq, PartialEq, Serialize)]
pub struct MetaSchema {
    #[serde(flatten)]
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(skip_serializing_if = "IndexSet::is_empty")]
    pub required: IndexSet<&'static str>,
    #[serde(skip_serializing_if = "IndexMap::is_empty")]
    pub properties: IndexMap<&'static str, MetaProperty>,
    pub deprecated: bool,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaProperty {
    #[serde(flatten)]
    pub data_type: DataType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MetaParamIn {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaOperationParam {
    pub name: &'static str,
    pub schema: DataType,
    #[serde(rename = "in")]
    pub in_type: MetaParamIn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    pub required: bool,
    pub deprecated: bool,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaMediaType {
    #[serde(skip)]
    pub content_type: &'static str,
    pub schema: MetaSchemaRef,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(
        skip_serializing_if = "<[_]>::is_empty",
        serialize_with = "serialize_content"
    )]
    pub content: &'static [MetaMediaType],
    pub required: bool,
}

fn serialize_content<S: Serializer>(
    content: &[MetaMediaType],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut s = serializer.serialize_map(None)?;
    for item in content {
        s.serialize_entry(item.content_type, item)?;
    }
    s.end()
}

#[derive(Debug, Eq, PartialEq)]
pub struct MetaResponses {
    pub responses: &'static [MetaResponse],
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(skip)]
    pub status: Option<u16>,
    #[serde(
        skip_serializing_if = "<[_]>::is_empty",
        serialize_with = "serialize_content"
    )]
    pub content: &'static [MetaMediaType],
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaOperation {
    #[serde(skip)]
    pub method: Method,
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub tags: &'static [&'static str],
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(rename = "parameters", skip_serializing_if = "<[_]>::is_empty")]
    pub params: &'static [MetaOperationParam],
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request: Option<&'static MetaRequest>,
    pub responses: &'static MetaResponses,
    pub deprecated: bool,
}

#[derive(Debug, Eq, PartialEq)]
pub struct MetaPath {
    pub path: &'static str,
    pub operations: &'static [MetaOperation],
}

#[derive(Debug, Eq, PartialEq)]
pub enum MetaSchemaRef {
    Inline(DataType),
    Reference(&'static str),
}

#[derive(Debug, Default, Eq, PartialEq, Serialize)]
pub struct MetaInfo {
    pub title: Option<&'static str>,
    pub description: Option<&'static str>,
    pub version: Option<&'static str>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaServer {
    pub url: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Debug, Eq, PartialEq, Serialize)]
pub struct MetaTag {
    pub name: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct MetaAPI {
    pub paths: &'static [MetaPath],
}

#[derive(Default)]
pub struct Registry {
    pub schemas: IndexMap<&'static str, MetaSchema>,
}

impl Registry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_schema<T, F>(&mut self, mut f: F)
    where
        T: Schema,
        F: FnMut(&mut Registry) -> MetaSchema,
    {
        let name = T::NAME;
        if !self.schemas.contains_key(name) {
            // Inserting a fake type before calling the function allows recursive types to
            // exist.
            self.schemas.insert(name, MetaSchema::default());
            let meta_schema = f(self);
            *self.schemas.get_mut(name).unwrap() = meta_schema;
        }
    }
}
