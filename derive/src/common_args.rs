use darling::FromMeta;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Meta, NestedMeta, Path};

#[derive(Debug, Copy, Clone, FromMeta)]
pub(crate) enum RenameRule {
    #[darling(rename = "lowercase")]
    Lower,
    #[darling(rename = "UPPERCASE")]
    Upper,
    #[darling(rename = "PascalCase")]
    Pascal,
    #[darling(rename = "camelCase")]
    Camel,
    #[darling(rename = "snake_case")]
    Snake,
    #[darling(rename = "SCREAMING_SNAKE_CASE")]
    ScreamingSnake,
}

impl RenameRule {
    pub(crate) fn rename(&self, name: impl AsRef<str>) -> String {
        match self {
            Self::Lower => name.as_ref().to_lowercase(),
            Self::Upper => name.as_ref().to_uppercase(),
            Self::Pascal => name.as_ref().to_pascal_case(),
            Self::Camel => name.as_ref().to_camel_case(),
            Self::Snake => name.as_ref().to_snake_case(),
            Self::ScreamingSnake => name.as_ref().to_screaming_snake_case(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum RenameTarget {
    Type,
    EnumItem,
    Field,
}

impl RenameTarget {
    pub(crate) fn rule(&self) -> RenameRule {
        match self {
            RenameTarget::Type => RenameRule::Pascal,
            RenameTarget::EnumItem => RenameRule::ScreamingSnake,
            RenameTarget::Field => RenameRule::Camel,
        }
    }

    pub(crate) fn rename(&self, name: impl AsRef<str>) -> String {
        self.rule().rename(name)
    }
}

pub(crate) trait RenameRuleExt {
    fn rename(&self, name: impl AsRef<str>, target: RenameTarget) -> String;
}

impl RenameRuleExt for Option<RenameRule> {
    fn rename(&self, name: impl AsRef<str>, target: RenameTarget) -> String {
        self.unwrap_or(target.rule()).rename(name)
    }
}

#[derive(FromMeta)]
pub(crate) struct ConcreteType {
    pub(crate) name: String,
    pub(crate) params: PathList,
}

pub(crate) struct PathList(pub(crate) Vec<Path>);

impl FromMeta for PathList {
    fn from_list(items: &[NestedMeta]) -> darling::Result<Self> {
        let mut res = Vec::new();
        for item in items {
            if let NestedMeta::Meta(Meta::Path(p)) = item {
                res.push(p.clone());
            } else {
                return Err(darling::Error::custom("Invalid path list"));
            }
        }
        Ok(PathList(res))
    }
}

#[derive(Debug, Copy, Clone, FromMeta)]
pub(crate) enum APIMethod {
    #[darling(rename = "get")]
    GET,
    #[darling(rename = "post")]
    POST,
    #[darling(rename = "put")]
    PUT,
    #[darling(rename = "delete")]
    DELETE,
    #[darling(rename = "head")]
    HEAD,
    #[darling(rename = "options")]
    OPTIONS,
    #[darling(rename = "connect")]
    CONNECT,
    #[darling(rename = "patch")]
    PATCH,
    #[darling(rename = "trace")]
    TRACE,
}

impl APIMethod {
    pub(crate) fn to_http_method(&self) -> TokenStream {
        match self {
            APIMethod::GET => quote!(GET),
            APIMethod::POST => quote!(POST),
            APIMethod::PUT => quote!(PUT),
            APIMethod::DELETE => quote!(DELETE),
            APIMethod::HEAD => quote!(HEAD),
            APIMethod::OPTIONS => quote!(OPTIONS),
            APIMethod::CONNECT => quote!(CONNECT),
            APIMethod::PATCH => quote!(PATCH),
            APIMethod::TRACE => quote!(TRACE),
        }
    }
}

#[derive(Debug, Copy, Clone, FromMeta, Eq, PartialEq)]
pub(crate) enum ParamIn {
    #[darling(rename = "path")]
    Path,
    #[darling(rename = "query")]
    Query,
    #[darling(rename = "header")]
    Header,
    #[darling(rename = "cookie")]
    Cookie,
}

impl ParamIn {
    pub(crate) fn to_meta(&self) -> TokenStream {
        match self {
            ParamIn::Path => quote!(Path),
            ParamIn::Query => quote!(Query),
            ParamIn::Header => quote!(Header),
            ParamIn::Cookie => quote!(Cookie),
        }
    }
}
