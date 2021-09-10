//! Macros for poem-openapi

#[macro_use]
mod validators;

mod api;
mod common_args;
mod r#enum;
mod error;
mod multipart;
mod object;
mod request;
mod response;
mod utils;

use proc_macro::TokenStream;
use syn::{parse_macro_input, AttributeArgs, DeriveInput, ItemImpl};

#[proc_macro_derive(Object, attributes(oai))]
pub fn derive_object(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match object::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Enum, attributes(oai))]
pub fn derive_enum(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match r#enum::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Response, attributes(oai))]
pub fn derive_response(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match response::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_derive(Request, attributes(oai))]
pub fn derive_request(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as DeriveInput);
    match request::generate(args) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}

#[proc_macro_attribute]
#[allow(non_snake_case)]
pub fn OpenApi(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let item_impl = parse_macro_input!(input as ItemImpl);
    match api::generate(args, item_impl) {
        Ok(stream) => stream.into(),
        Err(err) => err.write_errors().into(),
    }
}
