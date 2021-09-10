use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Attribute, DeriveInput, Error, Generics, Type};

use crate::{
    common_args::{RenameRule, RenameRuleExt, RenameTarget},
    error::GeneratorResult,
    utils::{get_crate_name, get_summary_and_description, optional_literal},
};

#[derive(FromField)]
#[darling(attributes(oai), forward_attrs(doc))]
struct MultipartField {
    ident: Option<Ident>,
    ty: Type,
    attrs: Vec<Attribute>,

    #[darling(default)]
    skip: bool,

    #[darling(default)]
    name: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(oai))]
struct MultipartArgs {
    ident: Ident,
    generics: Generics,
    data: Data<Ignored, MultipartField>,

    #[darling(default)]
    internal: bool,
    #[darling(default)]
    rename_fields: Option<RenameRule>,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let args: MultipartArgs = MultipartArgs::from_derive_input(&args)?;
    let crate_name = get_crate_name(args.internal);
    let (impl_generics, ty_generics, where_clause) = args.generics.split_for_impl();
    let ident = &args.ident;

    let s = match &args.data {
        Data::Struct(s) => s,
        _ => {
            return Err(
                Error::new_spanned(ident, "Multipart can only be applied to an struct.").into(),
            )
        }
    };

    let mut skip_fields = Vec::new();
    let mut skip_idents = Vec::new();
    let mut deserialize_fields = Vec::new();
    let mut deserialize_none = Vec::new();
    let mut fields = Vec::new();
    let mut meta_fields = Vec::new();
    let mut register_fields = Vec::new();
    let mut required_fields = Vec::new();

    for field in &s.fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        if field.skip {
            skip_fields.push(quote! {
                let #field_ident: #field_ty = ::std::default::Default::default();
            });
            skip_idents.push(field_ident);
            continue;
        }

        let field_name = field.name.clone().unwrap_or_else(|| {
            args.rename_fields
                .rename(field_ident.unraw().to_string(), RenameTarget::Field)
        });
        let (field_title, field_description) = get_summary_and_description(&field.attrs)?;
        let field_title = optional_literal(&field_title);
        let field_description = optional_literal(&field_description);

        fields.push(field_ident);

        deserialize_fields.push(quote! {
            if field.name() == ::std::option::Option::Some(#field_name) {
                let mut field_body = #crate_name::poem::RequestBody::new(#crate_name::poem::Body::from_async_read(field.into_async_read()));
                #field_ident = ::std::option::Option::Some(<#field_ty as #crate_name::payload::Payload>::from_request(&field_req, &mut field_body).await?);
                continue;
            }
        });

        deserialize_none.push(quote! {
            let #field_ident = match #field_ident {
                ::std::option::Option::Some(payload) => payload,
                ::std::option::Option::None => {
                    <#field_ty as #crate_name::payload::Payload>::from_request(&#crate_name::poem::Request::default(), &mut #crate_name::poem::RequestBody::default()).await.map_err(|_|
                        #crate_name::ParseRequestError::ParseRequestBody { reason: ::std::format!("field `{}` is required", #field_name) }
                    )?
                }
            };
        });

        meta_fields.push(quote! {{
            let mut schema_ref = <#field_ty as #crate_name::payload::Payload>::schema_ref();

            if let #crate_name::registry::MetaSchemaRef::Inline(schema) = &mut schema_ref {
                if let ::std::option::Option::Some(title) = #field_title {
                    schema.title = ::std::option::Option::Some(title);
                }

                if let ::std::option::Option::Some(field_description) = #field_description {
                    schema.description = ::std::option::Option::Some(field_description);
                }
            }

            (#field_name, schema_ref)
        }});

        register_fields.push(quote! {
            <#field_ty>::register(registry);
        });

        required_fields.push(quote! {
            if <#field_ty>::IS_REQUIRED {
                fields.push(#field_name);
            }
        });
    }

    let expanded = quote! {
        #[#crate_name::poem::async_trait]
        impl #impl_generics #crate_name::payload::Payload for #ident #ty_generics #where_clause {
            const CONTENT_TYPE: &'static str = "multipart/form-data";

            fn schema_ref() -> #crate_name::registry::MetaSchemaRef {
                let schema = #crate_name::registry::MetaSchema {
                    required: {
                        #[allow(unused_mut)]
                        let mut fields = ::std::vec::Vec::new();
                        #(#required_fields)*
                        fields
                    },
                    properties: ::std::vec![#(#meta_fields),*],
                    ..#crate_name::registry::MetaSchema::new("object")
                };
                #crate_name::registry::MetaSchemaRef::Inline(schema)
            }

            fn register(registry: &mut #crate_name::registry::Registry) {
                #(#register_fields)*
            }

            async fn from_request(request: &#crate_name::poem::Request, body: &mut #crate_name::poem::RequestBody) -> Result<Self, #crate_name::ParseRequestError> {
                if body.is_some() {
                    let mut multipart = <#crate_name::poem::web::Multipart as #crate_name::poem::FromRequest>::from_request(request, body).await.map_err(|err| #crate_name::ParseRequestError::ParseRequestBody {
                        reason: ::std::string::ToString::to_string(&err),
                    })?;
                    #(#skip_fields)*
                    #(let mut #fields = ::std::option::Option::None;)*
                    while let Some(field) = multipart.next_field().await.map_err(|err| #crate_name::ParseRequestError::ParseRequestBody { reason: ::std::string::ToString::to_string(&err) })? {
                        let mut field_req = #crate_name::poem::Request::builder();
                        if let Some(filename) = field.file_name() {
                            field_req = field_req.header("poem-filename", filename);
                        }
                        if let Some(content_type) = field.content_type() {
                            field_req = field_req.header(#crate_name::poem::http::header::CONTENT_TYPE, content_type);
                        }
                        let field_req = field_req.finish();
                        #(#deserialize_fields)*
                    }
                    #(#deserialize_none)*
                    ::std::result::Result::Ok(Self { #(#fields,)* #(#skip_idents),* })
                } else {
                    Err(#crate_name::ParseRequestError::ParseRequestBody {
                        reason: ::std::convert::Into::into("expect request body"),
                    })
                }
            }
        }
    };

    Ok(expanded)
}
