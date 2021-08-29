use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromVariant,
};
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote};
use syn::{Attribute, DeriveInput, Error, Result, Type};

use crate::{
    error::GeneratorResult,
    utils::{get_crate_name, get_description, optional_literal},
};

#[derive(FromVariant)]
#[darling(attributes(oai), forward_attrs(doc))]
struct ResponseItem {
    ident: Ident,
    attrs: Vec<Attribute>,
    fields: Fields<Type>,

    #[darling(default)]
    pub status: Option<u16>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(oai), forward_attrs(doc))]
struct ResponseArgs {
    ident: Ident,
    data: Data<ResponseItem, Ignored>,

    #[darling(default)]
    internal: bool,
    #[darling(default)]
    bad_request_handler: Option<String>,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let args: ResponseArgs = ResponseArgs::from_derive_input(&args)?;
    let crate_name = get_crate_name(args.internal);
    let ident = &args.ident;
    let e = match &args.data {
        Data::Enum(e) => e,
        _ => {
            return Err(
                Error::new_spanned(ident, "Response can only be applied to an enum.").into(),
            )
        }
    };

    let mut into_responses = Vec::new();
    let mut responses_meta = Vec::new();
    let mut schemas = Vec::new();

    for variant in e {
        let item_ident = &variant.ident;
        let item_description = get_description(&variant.attrs)?;
        let item_description = optional_literal(&item_description);

        match variant.fields.len() {
            2 => {
                // #[oai(default)]
                // Item(StatusCode, payload)
                let payload_ty = &variant.fields.fields[1];
                into_responses.push(quote! {
                    #ident::#item_ident(status, payload) => {
                        let mut resp = #crate_name::poem::IntoResponse::into_response(payload);
                        resp.set_status(status);
                        resp
                    }
                });
                responses_meta.push(quote! {
                    #crate_name::registry::MetaResponse {
                        description: #item_description,
                        status: ::std::option::Option::None,
                        content: &[#crate_name::registry::MetaMediaType {
                            content_type: <#payload_ty as #crate_name::payload::Payload>::CONTENT_TYPE,
                            schema: <#payload_ty as #crate_name::payload::Payload>::SCHEMA_REF,
                        }]
                    }
                });
                schemas.push(payload_ty);
            }
            1 => {
                // #[oai(status = 200)]
                // Item(payload)
                let payload_ty = &variant.fields.fields[0];
                let status = get_status(variant.ident.span(), variant.status)?;
                into_responses.push(quote! {
                    #ident::#item_ident(payload) => {
                        let mut resp = #crate_name::poem::IntoResponse::into_response(payload);
                        resp.set_status(#crate_name::poem::http::StatusCode::from_u16(#status).unwrap());
                        resp
                    }
                });
                responses_meta.push(quote! {
                    #crate_name::registry::MetaResponse {
                        description: #item_description,
                        status: ::std::option::Option::Some(#status),
                        content: &[#crate_name::registry::MetaMediaType {
                            content_type: <#payload_ty as #crate_name::payload::Payload>::CONTENT_TYPE,
                            schema: <#payload_ty as #crate_name::payload::Payload>::SCHEMA_REF,
                        }]
                    }
                });
                schemas.push(payload_ty);
            }
            0 => {
                // #[oai(status = 200)]
                // Item
                let status = get_status(variant.ident.span(), variant.status)?;
                into_responses.push(quote! {
                    #ident::#item_ident => {
                        let status = #crate_name::poem::http::StatusCode::from_u16(#status).unwrap();
                        #crate_name::poem::IntoResponse::into_response(status)
                    }
                });
                responses_meta.push(quote! {
                    #crate_name::registry::MetaResponse {
                        description: #item_description,
                        status: ::std::option::Option::Some(#status),
                        content: &[],
                    }
                });
            }
            _ => {
                return Err(
                    Error::new_spanned(&variant.ident, "Incorrect response definition.").into(),
                )
            }
        }
    }

    let bad_request_handler_const = match &args.bad_request_handler {
        Some(_) => quote!(
            const BAD_REQUEST_HANDLER: bool = true;
        ),
        None => quote!(
            const BAD_REQUEST_HANDLER: bool = false;
        ),
    };
    let bad_request_handler = match &args.bad_request_handler {
        Some(name) => {
            let name = format_ident!("{}", name);
            Some(quote! {
                fn from_parse_request_error(err: #crate_name::poem::Error) -> Self {
                    #name(err)
                }
            })
        }
        None => None,
    };

    let expanded = {
        quote! {
            impl #crate_name::poem::IntoResponse for #ident {
                fn into_response(self) -> #crate_name::poem::Response {
                    match self {
                        #(#into_responses)*
                    }
                }
            }

            impl #crate_name::Response for #ident {
                const META: &'static #crate_name::registry::MetaResponses = &#crate_name::registry::MetaResponses {
                    responses: &[#(#responses_meta),*],
                };
                #bad_request_handler_const

                fn register(registry: &mut #crate_name::registry::Registry) {
                    #(<#schemas as #crate_name::payload::Payload>::register(registry);)*
                }

                #bad_request_handler
            }
        }
    };

    Ok(expanded.into())
}

fn get_status(span: Span, status: Option<u16>) -> Result<TokenStream> {
    let status =
        status.ok_or_else(|| Error::new(span, "Response can only be applied to an enum."))?;
    if status < 100 || status >= 1000 {
        return Err(Error::new(
            span,
            "Invalid status code, it must be greater or equal to 100 and less than 1000.",
        ));
    }
    Ok(quote!(#status))
}
