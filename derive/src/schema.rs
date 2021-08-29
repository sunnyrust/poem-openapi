use darling::{ast::Data, util::Ignored, FromDeriveInput, FromField};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Attribute, DeriveInput, Error, Generics, Type};

use crate::{
    common_args::{ConcreteType, RenameRule, RenameRuleExt, RenameTarget},
    error::GeneratorResult,
    utils::{get_crate_name, get_description, get_summary_and_description, optional_literal},
};

#[derive(FromField)]
#[darling(attributes(oai), forward_attrs(doc))]
struct SchemaField {
    ident: Option<Ident>,
    ty: Type,
    attrs: Vec<Attribute>,

    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    skip: bool,
}

#[derive(FromDeriveInput)]
#[darling(attributes(oai), forward_attrs(doc))]
struct SchemaArgs {
    ident: Ident,
    generics: Generics,
    attrs: Vec<Attribute>,
    data: Data<Ignored, SchemaField>,

    #[darling(default)]
    internal: bool,
    #[darling(default)]
    name: Option<String>,
    #[darling(default)]
    rename_fields: Option<RenameRule>,
    #[darling(default, multiple, rename = "concrete")]
    concretes: Vec<ConcreteType>,
    #[darling(default)]
    deprecated: bool,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let args: SchemaArgs = SchemaArgs::from_derive_input(&args)?;
    let crate_name = get_crate_name(args.internal);
    let (impl_generics, ty_generics, where_clause) = args.generics.split_for_impl();
    let ident = &args.ident;
    let s = match &args.data {
        Data::Struct(s) => s,
        _ => {
            return Err(
                Error::new_spanned(ident, "Schema can only be applied to an struct.").into(),
            )
        }
    };
    let oai_typename = args
        .name
        .clone()
        .unwrap_or_else(|| RenameTarget::Type.rename(ident.to_string()));
    let (summary, description) = get_summary_and_description(&args.attrs)?;
    let mut deserialize_fields = Vec::new();
    let mut serialize_fields = Vec::new();
    let mut fields = Vec::new();
    let mut schema_fields = Vec::new();
    let mut required_fields = Vec::new();

    for field in &s.fields {
        let field_ident = field.ident.as_ref().unwrap();
        let field_ty = &field.ty;

        if field.skip {
            deserialize_fields.push(quote! {
                let #field_ident: #field_ty = ::std::default::Default::default();
            });
            fields.push(ident);
            continue;
        }

        let field_name = field.name.clone().unwrap_or_else(|| {
            args.rename_fields
                .rename(field_ident.unraw().to_string(), RenameTarget::Field)
        });
        let field_description = get_description(&field.attrs)?;

        fields.push(field_ident);

        deserialize_fields.push(quote! {
            #[allow(non_snake_case)]
            let #field_ident: #field_ty = #crate_name::types::Type::parse(obj.get(#field_name).cloned())
                .map_err(#crate_name::types::ParseError::propagate)?;
        });

        serialize_fields.push(quote! {
            let value = #crate_name::types::Type::to_value(&self.#field_ident);
            object.insert(::std::string::ToString::to_string(#field_name), value);
        });

        let field_description = optional_literal(&field_description);
        schema_fields.push(quote! {{
            <#field_ty>::register(registry);
            let property = #crate_name::registry::MetaProperty {
                data_type: <#field_ty>::DATA_TYPE,
                description: #field_description,
                default: ::std::option::Option::None,
            };
            (#field_name, property)
        }});

        required_fields.push(quote! {
            if <#field_ty>::IS_REQUIRED {
                fields.insert(#field_name);
            }
        });
    }

    let summary = optional_literal(&summary);
    let description = optional_literal(&description);
    let deprecated = args.deprecated;

    let register_type = quote! {
        registry.create_schema::<Self, _>(|registry| {
            #crate_name::registry::MetaSchema {
                data_type: Self::DATA_TYPE,
                summary: #summary,
                description: #description,
                required: {
                    let mut fields = #crate_name::indexmap::IndexSet::new();
                    #(#required_fields)*
                    fields
                },
                properties: ::std::iter::Iterator::collect(::std::iter::IntoIterator::into_iter([#(#schema_fields),*])),
                deprecated: #deprecated,
            }
        });
    };

    let expanded = if args.concretes.is_empty() {
        quote! {
            impl #crate_name::types::Type for #ident {
                const DATA_TYPE: #crate_name::types::DataType = #crate_name::types::DataType::new("object");

                fn parse(value: ::std::option::Option<#crate_name::serde_json::Value>) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
                    if let ::std::option::Option::Some(#crate_name::serde_json::Value::Object(obj)) = value {
                        #(#deserialize_fields)*
                        ::std::result::Result::Ok(Self { #(#fields),* })
                    } else {
                        ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value.unwrap_or_default()))
                    }
                }

                fn parse_from_str(value: ::std::option::Option<&str>) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
                    ::std::result::Result::Err(#crate_name::types::ParseError::not_support_parsing_from_string())
                }

                fn to_value(&self) -> #crate_name::serde_json::Value {
                    let mut object = ::serde_json::Map::new();
                    #(#serialize_fields)*
                    #crate_name::serde_json::Value::Object(object)
                }

                fn register(registry: &mut #crate_name::registry::Registry) {
                    #register_type
                }
            }

            impl #crate_name::Schema for #ident {
                const NAME: &'static str = #oai_typename;
            }

            impl #crate_name::serde::Serialize for #ident {
                fn serialize<S: #crate_name::serde::Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
                    #crate_name::types::Type::to_value(self).serialize(serializer)
                }
            }

            impl<'de> #crate_name::serde::Deserialize<'de> for #ident {
                fn deserialize<D: #crate_name::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
                    let value: #crate_name::serde_json::Value = #crate_name::serde::de::Deserialize::deserialize(deserializer)?;
                    #crate_name::types::Type::parse(::std::option::Option::Some(value)).map_err(|err| #crate_name::serde::de::Error::custom(err.into_message()))
                }
            }
        }
    } else {
        let mut code = Vec::new();

        code.push(quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                fn __internal_parse(value: #crate_name::serde_json::Value) -> #crate_name::ParseResult<Self> {
                    if let ::std::option::Option::Some(#crate_name::serde_json::Value::Object(obj)) = value {
                        #(#deserialize_fields)*
                        ::std::result::Result::Ok(Self { #(#fields),* })
                    } else {
                        ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value.unwrap_or_default()))
                    }
                }

                fn __internal_to_value(&self) -> #crate_name::serde_json::Value {
                    let mut object = ::serde_json::Map::new();
                    #(#serialize_fields)*
                    #crate_name::serde_json::Value::Object(object)
                }
            }
        });

        for concrete in &args.concretes {
            let oai_typename = &concrete.name;
            let params = &concrete.params.0;
            let concrete_type = quote! { #ident<#(#params),*> };

            let expanded = quote! {
                impl #crate_name::types::Type for #concrete_type {
                    const DATA_TYPE: #crate_name::types::DataType = #crate_name::types::DataType::new(#oai_typename);

                    fn parse(value: #crate_name::serde_json::Value) -> #crate_name::ParseResult<Self> {
                        Self::__internal_parse(value)
                    }

                    fn to_value(&self) -> #crate_name::serde_json::Value {
                        Self::__internal_to_value()
                    }

                    fn register(registry: &mut #crate_name::registry::Registry) {
                        #register_type
                    }
                }

                impl #crate_name::SchemaType for #concrete_type {
                    const NAME: &'static str = #oai_typename;
                }
            };
            code.push(expanded);
        }

        quote!(#(#code)*)
    };

    Ok(expanded.into())
}
