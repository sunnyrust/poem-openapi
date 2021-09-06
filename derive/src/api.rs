use darling::{util::SpannedValue, FromMeta};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{AttributeArgs, Error, FnArg, ImplItem, ImplItemMethod, ItemImpl, ReturnType};

use crate::{
    common_args::{APIMethod, DefaultValue, MaximumValidator, MinimumValidator, ParamIn},
    error::GeneratorResult,
    utils::{
        convert_oai_path, get_crate_name, get_summary_and_description, optional_literal,
        parse_oai_attrs, remove_oai_attrs,
    },
    validators::HasValidators,
};

#[derive(FromMeta)]
struct APIArgs {
    #[darling(default)]
    internal: bool,
}

#[derive(FromMeta)]
struct APIOperation {
    path: SpannedValue<String>,
    method: APIMethod,
    #[darling(default)]
    deprecated: bool,
    #[darling(default, multiple, rename = "tag")]
    tags: Vec<String>,
}

#[derive(FromMeta, Default)]
struct APIOperationParam {
    #[darling(default)]
    name: Option<String>,
    #[darling(default, rename = "in")]
    param_in: Option<ParamIn>,
    #[darling(default)]
    extract: bool,
    #[darling(default)]
    desc: Option<String>,
    #[darling(default)]
    deprecated: bool,
    #[darling(default)]
    default: Option<DefaultValue>,

    #[darling(default)]
    multiple_of: Option<SpannedValue<f64>>,
    #[darling(default)]
    maximum: Option<SpannedValue<MaximumValidator>>,
    #[darling(default)]
    minimum: Option<SpannedValue<MinimumValidator>>,
    #[darling(default)]
    max_length: Option<SpannedValue<usize>>,
    #[darling(default)]
    min_length: Option<SpannedValue<usize>>,
    #[darling(default)]
    pattern: Option<SpannedValue<String>>,
    #[darling(default)]
    max_items: Option<SpannedValue<usize>>,
    #[darling(default)]
    min_items: Option<SpannedValue<usize>>,
}

impl_has_validators!(APIOperationParam);

struct Context {
    add_routes: IndexMap<String, Vec<TokenStream>>,
    operations: IndexMap<String, Vec<TokenStream>>,
    request_types: Vec<TokenStream>,
    response_types: Vec<TokenStream>,
}

pub(crate) fn generate(
    args: AttributeArgs,
    mut item_impl: ItemImpl,
) -> GeneratorResult<TokenStream> {
    let APIArgs { internal } = match APIArgs::from_list(&args) {
        Ok(args) => args,
        Err(err) => return Ok(err.write_errors().into()),
    };
    let crate_name = get_crate_name(internal);
    let ident = item_impl.self_ty.clone();
    let mut ctx = Context {
        add_routes: Default::default(),
        operations: Default::default(),
        request_types: Default::default(),
        response_types: Default::default(),
    };

    for item in &mut item_impl.items {
        if let ImplItem::Method(method) = item {
            if let Some(operation_args) = parse_oai_attrs::<APIOperation>(&method.attrs)? {
                if method.sig.asyncness.is_none() {
                    return Err(
                        Error::new_spanned(&method.sig.ident, "Must be asynchronous").into(),
                    );
                }

                generate_operation(&mut ctx, &crate_name, operation_args, method)?;
                remove_oai_attrs(&mut method.attrs);
            }
        }
    }

    let Context {
        add_routes,
        operations,
        request_types,
        response_types,
        ..
    } = ctx;

    let paths = {
        let mut paths = Vec::new();

        for (path, operation) in operations {
            paths.push(quote! {
                #crate_name::registry::MetaPath {
                    path: #path,
                    operations: ::std::vec![#(#operation),*],
                }
            });
        }
        paths
    };

    let routes = {
        let mut routes = Vec::new();

        for (path, add_route) in add_routes {
            routes.push(quote! {
                at(#path, #crate_name::poem::RouteMethod::new()#(.#add_route)*)
            });
        }

        routes
    };

    let register_schemas = {
        let mut register_schemas = Vec::new();

        for ty in request_types {
            register_schemas.push(quote!(<#ty as #crate_name::Request>::register(registry);));
        }
        for ty in response_types {
            register_schemas.push(quote!(<#ty as #crate_name::Response>::register(registry);));
        }

        register_schemas
    };

    let expanded = quote! {
        #item_impl

        impl #crate_name::API for #ident {
            fn meta() -> ::std::vec::Vec<#crate_name::registry::MetaAPI> {
                ::std::vec![#crate_name::registry::MetaAPI {
                    paths: ::std::vec![#(#paths),*],
                }]
            }

            fn register(registry: &mut #crate_name::registry::Registry) {
                #(#register_schemas)*
            }

            fn add_routes(self, route: #crate_name::poem::route::Route) -> #crate_name::poem::route::Route {
                let api_obj = ::std::sync::Arc::new(self);
                route#(.#routes)*
            }
        }
    };

    Ok(expanded.into())
}

fn generate_operation(
    ctx: &mut Context,
    crate_name: &TokenStream,
    args: APIOperation,
    item_method: &mut ImplItemMethod,
) -> GeneratorResult<()> {
    let APIOperation {
        path,
        method,
        deprecated,
        tags,
    } = args;
    let http_method = method.to_http_method();
    let fn_ident = &item_method.sig.ident;
    let (summary, description) = get_summary_and_description(&item_method.attrs)?;
    let summary = optional_literal(&summary);
    let description = optional_literal(&description);

    let (oai_path, new_path, path_vars) = convert_oai_path(&path)?;

    if item_method.sig.inputs.is_empty() {
        return Err(Error::new_spanned(
            &item_method.sig.inputs,
            "At least one `&self` receiver is required.",
        )
        .into());
    }

    if let FnArg::Receiver(receiver) = &item_method.sig.inputs[0] {
        if receiver.mutability.is_some() {
            return Err(Error::new_spanned(
                receiver,
                "The first parameter must be a `&self` receiver.",
            )
            .into());
        }
    } else {
        return Err(Error::new_spanned(
            &item_method.sig.inputs[0],
            "The first parameter must be a `&self` receiver.",
        )
        .into());
    }

    let res_ty = match &item_method.sig.output {
        ReturnType::Default => Box::new(syn::parse2(quote!(())).unwrap()),
        ReturnType::Type(_, ty) => ty.clone(),
    };

    let mut parse_args = Vec::new();
    let mut use_args = Vec::new();
    let mut has_request_payload = false;
    let mut request_meta = quote!(::std::option::Option::None);
    let mut params_meta = Vec::new();

    for i in 1..item_method.sig.inputs.len() {
        let arg = &mut item_method.sig.inputs[i];
        let pat = match arg {
            FnArg::Typed(pat) => pat,
            FnArg::Receiver(_) => {
                return Err(Error::new_spanned(item_method, "Invalid method definition.").into());
            }
        };
        let pname = format_ident!("p{}", i);
        let arg_ty = &pat.ty;

        let operation_param = parse_oai_attrs::<APIOperationParam>(&pat.attrs)?;
        remove_oai_attrs(&mut pat.attrs);

        match operation_param {
            // is poem extractor
            Some(operation_param) if operation_param.extract => {
                parse_args.push(quote! {
                    let #pname = match <#arg_ty as #crate_name::poem::FromRequest>::from_request(&request, &mut body).await.map_err(#crate_name::poem::Error::from) {
                        ::std::result::Result::Ok(value) => value,
                        ::std::result::Result::Err(err) if <#res_ty as #crate_name::Response>::BAD_REQUEST_HANDLER => {
                                return ::std::result::Result::Ok(<#res_ty as #crate_name::Response>::from_parse_request_error(err));
                            },
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    };
                });
                use_args.push(pname);
            }

            // is parameter
            Some(operation_param) => {
                let param_oai_typename = match &operation_param.name {
                    Some(name) => name.clone(),
                    None => {
                        return Err(Error::new_spanned(
                            arg,
                            r#"The request parameter is missing a name. #[oai(name = "...")]"#,
                        )
                        .into())
                    }
                };

                let param_in =
                    match operation_param.param_in {
                        Some(param_in) => param_in,
                        None => return Err(Error::new_spanned(
                            arg,
                            r#"The request parameter is missing a input type. #[oai(in = "...")]"#,
                        )
                        .into()),
                    };

                if param_in == ParamIn::Path {
                    if !path_vars.contains(&*param_oai_typename) {
                        return Err(Error::new_spanned(
                            arg,
                            format!(
                                "The parameter `{}` is not defined in the path.",
                                param_oai_typename
                            ),
                        )
                        .into());
                    }
                }

                let meta_in = {
                    let ty = param_in.to_meta();
                    quote!(#crate_name::registry::MetaParamIn::#ty)
                };
                let validators_checker = operation_param
                    .validators()
                    .create_param_checker(&crate_name, &param_oai_typename)?;
                let validators_update_meta = operation_param
                    .validators()
                    .create_update_meta(&crate_name, &arg_ty)?;

                match &operation_param.default {
                    Some(default_value) => {
                        let default_value = match default_value {
                            DefaultValue::Default => {
                                quote!(<#arg_ty as ::std::default::Default>::default())
                            }
                            DefaultValue::Function(func_name) => quote!(#func_name()),
                        };

                        parse_args.push(quote! {
                            let #pname = {
                                let value = #crate_name::param::get(#param_oai_typename, #meta_in, &request, &query.0);
                                let value = value.as_deref();
                                match value {
                                    Some(value) => {
                                        match #crate_name::types::Type::parse_from_str(Some(value))
                                                .map_err(|err| #crate_name::poem::Error::bad_request(#crate_name::ParseRequestError::ParseParam {
                                                    name: #param_oai_typename,
                                                    reason: err.into_message(),
                                                }))
                                        {
                                            ::std::result::Result::Ok(value) => {
                                                #validators_checker
                                                value
                                            },
                                            ::std::result::Result::Err(err) if <#res_ty as #crate_name::Response>::BAD_REQUEST_HANDLER => {
                                                return ::std::result::Result::Ok(<#res_ty as #crate_name::Response>::from_parse_request_error(err));
                                            },
                                            ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                                        }
                                    }
                                    None => #default_value,
                                }
                            };
                        });
                    }
                    None => {
                        parse_args.push(quote! {
                            let #pname = {
                                let value = #crate_name::param::get(#param_oai_typename, #meta_in, &request, &query.0);
                                match #crate_name::types::Type::parse_from_str(value.as_deref())
                                        .map_err(|err| #crate_name::poem::Error::bad_request(#crate_name::ParseRequestError::ParseParam {
                                            name: #param_oai_typename,
                                            reason: err.into_message(),
                                        }))
                                {
                                    ::std::result::Result::Ok(value) => {
                                        #validators_checker
                                        value
                                    },
                                    ::std::result::Result::Err(err) if <#res_ty as #crate_name::Response>::BAD_REQUEST_HANDLER => {
                                        return ::std::result::Result::Ok(<#res_ty as #crate_name::Response>::from_parse_request_error(err));
                                    },
                                    ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                                }
                            };
                        });
                    }
                }

                let meta_arg_default = match &operation_param.default {
                    Some(DefaultValue::Default) => quote! {
                        ::std::option::Option::Some(<#arg_ty as ::std::default::Default>::default().to_value())
                    },
                    Some(DefaultValue::Function(func_name)) => quote! {
                        ::std::option::Option::Some(#func_name().to_value())
                    },
                    None => quote!(::std::option::Option::None),
                };

                use_args.push(pname);

                let desc = optional_literal(&operation_param.desc);
                let deprecated = operation_param.deprecated;
                params_meta.push(quote! {
                    #[allow(unused_mut)]
                    #crate_name::registry::MetaOperationParam {
                        name: #param_oai_typename,
                        schema: {
                            let mut schema_ref = <#arg_ty as #crate_name::types::Type>::schema_ref();

                            if let #crate_name::registry::MetaSchemaRef::Inline(schema) = &mut schema_ref {
                                schema.default = #meta_arg_default;
                                #validators_update_meta
                            }

                            schema_ref
                        },
                        in_type: #meta_in,
                        description: #desc,
                        required: <#arg_ty as #crate_name::types::Type>::IS_REQUIRED,
                        deprecated: #deprecated,
                    }
                });
            }

            // is request body
            None => {
                if has_request_payload {
                    return Err(
                        Error::new_spanned(arg, "Only one request payload is allowed.").into(),
                    );
                }

                parse_args.push(quote! {
                    let #pname = match <#arg_ty as #crate_name::Request>::from_request(&request, &mut body).await {
                        ::std::result::Result::Ok(value) => value,
                        ::std::result::Result::Err(err) if <#res_ty as #crate_name::Response>::BAD_REQUEST_HANDLER => {
                                return ::std::result::Result::Ok(<#res_ty as #crate_name::Response>::from_parse_request_error(err));
                            },
                        ::std::result::Result::Err(err) => return ::std::result::Result::Err(err),
                    };
                });
                use_args.push(pname);

                has_request_payload = true;
                request_meta =
                    quote!(::std::option::Option::Some(<#arg_ty as #crate_name::Request>::meta()));
                ctx.request_types.push(quote!(#arg_ty));
            }
        }
    }

    ctx.response_types.push(quote!(#res_ty));

    ctx.add_routes.entry(new_path).or_default().push(quote! {
        method(#crate_name::poem::http::Method::#http_method, {
            let api_obj = ::std::clone::Clone::clone(&api_obj);
            #crate_name::poem::endpoint::make(move |request| {
                let api_obj = ::std::clone::Clone::clone(&api_obj);
                async move {
                    let (request, mut body) = request.split();
                    let query = <#crate_name::poem::web::Query::<::std::collections::HashMap<::std::string::String, ::std::string::String>> as #crate_name::poem::FromRequest>::from_request(&request, &mut body).await.unwrap_or_default();
                    #(#parse_args)*
                    ::std::result::Result::Ok::<_, #crate_name::poem::Error>(api_obj.#fn_ident(#(#use_args),*).await)
                }
            })
        })
    });

    ctx.operations.entry(oai_path).or_default().push(quote! {
        #crate_name::registry::MetaOperation {
            tags: ::std::vec![#(#tags),*],
            method: #crate_name::poem::http::Method::#http_method,
            summary: #summary,
            description: #description,
            params: ::std::vec![#(#params_meta),*],
            request: #request_meta,
            responses: <#res_ty as #crate_name::Response>::meta(),
            deprecated: #deprecated,
        }
    });

    Ok(())
}
