//! OpenAPI support for Poem.

#![forbid(unsafe_code)]
#![deny(private_in_public, unreachable_pub)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

mod base;
mod error;
mod openapi;
#[doc(hidden)]
pub mod param;
pub mod payload;
#[doc(hidden)]
pub mod registry;
pub mod types;
#[doc(hidden)]
pub mod ui;

pub use base::{CombinedAPI, Request, Response, Schema, API};
pub use error::ParseRequestError;
#[doc(hidden)]
pub use indexmap;
pub use openapi::OpenAPI;
#[doc(hidden)]
pub use poem;
#[doc(hidden)]
pub use serde;
#[doc(hidden)]
pub use serde_json;

/// Define a OpenAPI enum
///
/// # Macro parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | name          | Object name               | string   | Y        |
/// | rename_items | Rename all the items according to the given case convention. The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE". | string   | Y        |
///
/// # Item parameters
///
/// | Attribute   | description               | Type     | Optional |
/// |-------------|---------------------------|----------|----------|
/// | name        | Item name                 | string   | Y        |
///
/// # Examples
///
/// ```
/// use poem_openapi::Enum;
///
/// #[derive(Enum)]
/// enum PetStatus {
///     Available,
///     Pending,
///     Sold,
/// }
/// ```
#[rustfmt::skip]
pub use poem_openapi_derive::Enum;

/// Define a OpenAPI request.
///
/// # Examples
///
/// ```
/// use poem_openapi::{
///     payload::{Json, PlainText},
///     Request, Schema,
/// };
/// 
/// #[derive(Schema)]
/// struct Pet {
///     id: String,
///     name: String,
/// }
///
/// #[derive(Request)]
/// enum CreatePet {
///     /// This request receives a pet in JSON format(application/json).
///     CreateByJSON(Json<Pet>),
///     /// This request receives a pet in text format(text/plain).
///     CreateByPlainText(PlainText),
/// }
/// ```
#[rustfmt::skip]
pub use poem_openapi_derive::Request;

/// Define a OpenAPI response.
///
/// # Macro parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | bad_request_handler | Sets a custom bad request handler, it can convert error to the value of the this response type. | string   | Y
///
/// # Item parameters
///
/// | Attribute   | description               | Type     | Optional |
/// |-------------|---------------------------|----------|----------|
/// | status      | HTTP status code. If omitted, it is a default response type. | u16   | Y        |
/// 
/// # Examples
/// 
/// ```
/// use poem_openapi::{payload::PlainText, Response};
/// use poem::Error;
///
/// #[derive(Response)]
/// #[oai(bad_request_handler = "bad_request_handler")]
/// enum CreateUserResponse {
///     /// Returns when the user is successfully created.
///     #[oai(status = 200)]
///     Ok,
///     /// Returns when the user already exists.
///     #[oai(status = 409)]
///     UserAlreadyExists,
///     /// Returns when the request parameters is incorrect.
///     #[oai(status = 400)]
///     BadRequest(PlainText),
/// }
/// 
/// // Convert error to `CreateUserResponse::BadRequest`.
/// fn bad_request_handler(err: Error) -> CreateUserResponse {
///     CreateUserResponse::BadRequest(PlainText(format!("error: {}", err)))
/// }
/// ```
#[rustfmt::skip]
pub use poem_openapi_derive::Response;

/// Define a OpenAPI schema
///
/// # Macro parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | name          | Object name               | string   | Y        |
/// | rename_fields | Rename all the fields according to the given case convention. The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE". | string   | Y        |
/// | concretes     | Specify how the concrete type of the generic Schema should be implemented. | ConcreteType |  Y |
/// | deprecation   | Schema deprecated          | bool     | Y        |
///
/// # Field parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | skip          | Skip this field           | bool     | Y        |
/// | name          | Field name                | string   | Y        |
/// 
/// # Examples
/// 
/// ```
/// use poem_openapi::Schema;
/// 
/// /// Pet
/// #[derive(Schema)]
/// struct Pet {
///     /// The id of this pet.
///     id: String,
/// 
///     /// The name of this pet.
///     name: String,
/// }
/// ```
#[rustfmt::skip]
pub use poem_openapi_derive::Schema;

/// Define a OpenAPI schema
///
/// # Operation parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | path          | HTTP uri.                 | string   | N        |
/// | method        | HTTP method. The possible values are "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace". | string   | N        |
/// | deprecation   | Operation deprecated      | bool     | Y        |
/// | tag           | Operation tag             | string   | Y        |
///
/// # Operation argument parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | name          | Parameter name. When this value is set, it means that this is an OpenAPI parameter type.           | string   | Y        |
/// | in            | Where to parse the parameter. The possible values are "query", "path", "header", "cookie". | string   | Y        |
/// | extract       | It means that this parameter is a Poem extractor. | bool | Y |
/// | desc          | Argument description      | string   | Y        |
/// | deprecation   | Argument deprecated       | bool     | Y        |
///
/// # Examples
/// 
/// ```
/// use poem_openapi::{
///     payload::{Json, PlainText},
///     Request, Schema, API, Response,
/// };
///
/// #[derive(Schema)]
/// struct Pet {
///     id: String,
///     name: String,
/// }
///
/// #[derive(Request)]
/// enum CreatePetRequest {
///     /// This request receives a pet in JSON format(application/json).
///     CreateByJSON(Json<Pet>),
///     /// This request receives a pet in text format(text/plain).
///     CreateByPlainText(PlainText),
/// }
///
/// #[derive(Response)]
/// enum CreatePetResponse {
///     /// Returns when the pet is successfully created.
///     #[oai(status = 200)]
///     Ok,
///     /// Returns when the pet already exists.
///     #[oai(status = 409)]
///     PetAlreadyExists,
/// }
/// 
/// struct PetAPI;
/// 
/// #[API]
/// impl PetAPI {
///     /// Create a new pet.
///     #[oai(path = "/pet", method = "post")]
///     async fn create_pet(
///         &self,
///         #[oai(name = "TOKEN", in = "header")] token: String,
///         req: CreatePetRequest
///     ) -> CreatePetResponse {
///         todo!() 
///     }
/// }
/// ```
#[rustfmt::skip]
pub use poem_openapi_derive::API;
