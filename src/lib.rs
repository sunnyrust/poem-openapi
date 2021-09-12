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
#[doc(hidden)]
pub mod validation;

pub use base::{CombinedAPI, OpenApi, Request, Response};
pub use error::ParseRequestError;
pub use openapi::OpenApiService;
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
///     Request, Object,
/// };
/// 
/// #[derive(Object)]
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
/// use poem_openapi::{payload::PlainText, Response, ParseRequestError};
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
/// fn bad_request_handler(err: ParseRequestError) -> CreateUserResponse {
///     CreateUserResponse::BadRequest(PlainText(format!("error: {}", err)))
/// }
/// ```
#[rustfmt::skip]
pub use poem_openapi_derive::Response;

/// Define a OpenAPI object
///
/// # Macro parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | name          | Object name               | string   | Y        |
/// | rename_fields | Rename all the fields according to the given case convention. The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE". | string   | Y        |
/// | concretes     | Specify how the concrete type of the generic Schema should be implemented. | ConcreteType |  Y |
/// | deprecated    | Schema deprecated          | bool     | Y        |
///
/// # Field parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | skip          | Skip this field           | bool     | Y        |
/// | name          | Field name                | string   | Y        |
/// | multiple_of   | The value of "multiple_of" MUST be a number, strictly greater than 0. A numeric instance is only valid if division by this value results in an integer. | number | Y |
/// | maximum       | The value of "maximum" MUST be a number, representing an upper limit for a numeric instance. If `exclusive` is `true` and instance is less than the provided value, or else if the instance is less than or exactly equal to the provided value. | { value: `<number>`, exclusive: `<bool>`} | Y |
/// | minimum       | The value of "minimum" MUST be a number, representing a lower limit for a numeric instance. If `exclusive` is `true` and instance is greater than the provided value, or else if the instance is greater than or exactly equal to the provided value. | { value: `<number>`, exclusive: `<bool>`} | Y |
/// | max_length    | The value of "max_length" MUST be a non-negative integer. A string instance is valid against this validator if its length is less than, or equal to, the value. | usize | Y |
/// | min_length    | The value of "min_length" MUST be a non-negative integer.  The value of this validator MUST be an integer. This integer MUST be greater than, or equal to, 0.| usize | Y |
/// | pattern       | The value of "pattern" MUST be a string. This string SHOULD be a valid regular expression, according to the ECMA 262 regular expression dialect. A string instance is considered valid if the regular expression matches the instance successfully. | string | Y |
/// | max_items     | The value of "max_items" MUST be an integer. This integer MUST be greater than, or equal to, 0. An array instance is valid if its size is less than, or equal to, the value of this validator. | usize | Y |
/// | min_items     | The value of "min_items" MUST be an integer. This integer MUST be greater than, or equal to, 0. An array instance is valid if its size is greater than, or equal to, the value of this validator. | usize | Y |
/// 
/// # Examples
/// 
/// ```
/// use poem_openapi::Object;
/// 
/// /// Pet
/// #[derive(Object)]
/// struct Pet {
///     /// The id of this pet.
///     id: String,
/// 
///     /// The name of this pet.
///     name: String,
/// }
/// ```
#[rustfmt::skip]
pub use poem_openapi_derive::Object;

#[rustfmt::skip]
pub use poem_openapi_derive::Multipart;

/// Define a OpenAPI.
///
/// # Operation parameters
///
/// | Attribute     | description               | Type     | Optional |
/// |---------------|---------------------------|----------|----------|
/// | path          | HTTP uri.                 | string   | N        |
/// | method        | HTTP method. The possible values are "get", "post", "put", "delete", "head", "options", "connect", "patch", "trace". | string   | N        |
/// | deprecated    | Operation deprecated      | bool     | Y        |
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
/// | deprecated    | Argument deprecated       | bool     | Y        |
/// | multiple_of   | The value of "multiple_of" MUST be a number, strictly greater than 0. A numeric instance is only valid if division by this value results in an integer. | number | Y |
/// | maximum       | The value of "maximum" MUST be a number, representing an upper limit for a numeric instance. If `exclusive` is `true` and instance is less than the provided value, or else if the instance is less than or exactly equal to the provided value. | { value: `<number>`, exclusive: `<bool>`} | Y |
/// | minimum       | The value of "minimum" MUST be a number, representing a lower limit for a numeric instance. If `exclusive` is `true` and instance is greater than the provided value, or else if the instance is greater than or exactly equal to the provided value. | { value: `<number>`, exclusive: `<bool>`} | Y |
/// | max_length    | The value of "max_length" MUST be a non-negative integer. A string instance is valid against this validator if its length is less than, or equal to, the value. | usize | Y |
/// | min_length    | The value of "min_length" MUST be a non-negative integer.  The value of this validator MUST be an integer. This integer MUST be greater than, or equal to, 0.| usize | Y |
/// | pattern       | The value of "pattern" MUST be a string. This string SHOULD be a valid regular expression, according to the ECMA 262 regular expression dialect. A string instance is considered valid if the regular expression matches the instance successfully. | string | Y |
/// | max_items     | The value of "max_items" MUST be an integer. This integer MUST be greater than, or equal to, 0. An array instance is valid if its size is less than, or equal to, the value of this validator. | usize | Y |
/// | min_items     | The value of "min_items" MUST be an integer. This integer MUST be greater than, or equal to, 0. An array instance is valid if its size is greater than, or equal to, the value of this validator. | usize | Y |
///
/// # Examples
/// 
/// ```
/// use poem_openapi::{
///     payload::{Json, PlainText},
///     Request, Object, OpenApi, Response,
/// };
///
/// #[derive(Object)]
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
/// struct PetApi;
///
/// #[OpenApi]
/// impl PetApi {
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
pub use poem_openapi_derive::OpenApi;
