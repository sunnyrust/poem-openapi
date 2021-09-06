mod ser;

use std::collections::HashMap;

use poem::http::Method;
pub(crate) use ser::Document;
use serde::{ser::SerializeMap, Serialize, Serializer};
use serde_json::Value;

use crate::types::TypeName;

#[inline]
fn is_false(value: &bool) -> bool {
    !*value
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MetaSchema {
    #[serde(rename = "type")]
    pub ty: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub required: Vec<&'static str>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "serialize_properties"
    )]
    pub properties: Vec<(&'static str, MetaSchemaRef)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<MetaSchemaRef>>,
    #[serde(rename = "enum", skip_serializing_if = "Vec::is_empty")]
    pub enum_items: Vec<Value>,
    #[serde(skip_serializing_if = "is_false")]
    pub deprecated: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_of: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_maximum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusive_minimum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_length: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pattern: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_items: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_items: Option<usize>,
}

fn serialize_properties<S: Serializer>(
    properties: &[(&'static str, MetaSchemaRef)],
    serializer: S,
) -> Result<S::Ok, S::Error> {
    let mut s = serializer.serialize_map(None)?;
    for item in properties {
        s.serialize_entry(item.0, &item.1)?;
    }
    s.end()
}

impl MetaSchema {
    pub const fn new(ty: &'static str) -> Self {
        Self {
            ty,
            format: None,
            title: None,
            description: None,
            default: None,
            required: vec![],
            properties: vec![],
            items: None,
            enum_items: vec![],
            deprecated: false,
            multiple_of: None,
            maximum: None,
            exclusive_maximum: None,
            minimum: None,
            exclusive_minimum: None,
            max_length: None,
            min_length: None,
            pattern: None,
            max_items: None,
            min_items: None,
        }
    }
}

impl From<TypeName> for MetaSchema {
    fn from(name: TypeName) -> Self {
        if let TypeName::Normal { ty, format } = name {
            let mut schema = MetaSchema::new(ty);
            schema.format = format;
            schema
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetaSchemaRef {
    Inline(MetaSchema),
    Reference(&'static str),
}

impl MetaSchemaRef {
    /// THIS FUNCTION ONLY FOR TESTS
    pub fn unwrap_inline(&self) -> &MetaSchema {
        match &self {
            MetaSchemaRef::Inline(schema) => schema,
            MetaSchemaRef::Reference(_) => panic!(),
        }
    }

    /// THIS FUNCTION ONLY FOR TESTS
    pub fn unwrap_reference(&self) -> &'static str {
        match self {
            MetaSchemaRef::Inline(_) => panic!(),
            MetaSchemaRef::Reference(name) => name,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MetaParamIn {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct MetaOperationParam {
    pub name: &'static str,
    pub schema: MetaSchemaRef,
    #[serde(rename = "in")]
    pub in_type: MetaParamIn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    pub required: bool,
    pub deprecated: bool,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct MetaMediaType {
    #[serde(skip)]
    pub content_type: &'static str,
    pub schema: MetaSchemaRef,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct MetaRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "serialize_content"
    )]
    pub content: Vec<MetaMediaType>,
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

#[derive(Debug, PartialEq)]
pub struct MetaResponses {
    pub responses: Vec<MetaResponse>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct MetaResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(skip)]
    pub status: Option<u16>,
    #[serde(
        skip_serializing_if = "Vec::is_empty",
        serialize_with = "serialize_content"
    )]
    pub content: Vec<MetaMediaType>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct MetaOperation {
    #[serde(skip)]
    pub method: Method,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<&'static str>,
    #[serde(rename = "parameters", skip_serializing_if = "Vec::is_empty")]
    pub params: Vec<MetaOperationParam>,
    #[serde(rename = "requestBody", skip_serializing_if = "Option::is_none")]
    pub request: Option<MetaRequest>,
    pub responses: MetaResponses,
    #[serde(rename = "parameters", skip_serializing_if = "is_false")]
    pub deprecated: bool,
}

#[derive(Debug, PartialEq)]
pub struct MetaPath {
    pub path: &'static str,
    pub operations: Vec<MetaOperation>,
}

#[derive(Debug, Default, PartialEq, Serialize)]
pub struct MetaInfo {
    pub title: Option<&'static str>,
    pub description: Option<&'static str>,
    pub version: Option<&'static str>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct MetaServer {
    pub url: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct MetaTag {
    pub name: &'static str,
    pub description: Option<&'static str>,
}

#[derive(Debug, PartialEq)]
pub struct MetaAPI {
    pub paths: Vec<MetaPath>,
}

#[derive(Default)]
pub struct Registry {
    pub schemas: HashMap<&'static str, MetaSchema>,
}

impl Registry {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn create_schema<F>(&mut self, name: &'static str, mut f: F)
    where
        F: FnMut(&mut Registry) -> MetaSchema,
    {
        if !self.schemas.contains_key(name) {
            // Inserting a fake type before calling the function allows recursive types to
            // exist.
            self.schemas.insert(name, MetaSchema::new("fake"));
            let meta_schema = f(self);
            *self.schemas.get_mut(name).unwrap() = meta_schema;
        }
    }
}
