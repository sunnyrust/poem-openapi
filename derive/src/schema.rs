use darling::{
    ast::Data,
    util::{Ignored, SpannedValue},
    FromDeriveInput, FromField, FromMeta,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use regex::Regex;
use syn::{ext::IdentExt, Attribute, DeriveInput, Error, Generics, Type};

use crate::{
    common_args::{ConcreteType, DefaultValue, RenameRule, RenameRuleExt, RenameTarget},
    error::GeneratorResult,
    utils::{get_crate_name, get_summary_and_description, optional_literal},
};

#[derive(FromMeta)]
struct MaximumValidator {
    value: f64,
    #[darling(default)]
    exclusive: bool,
}

#[derive(FromMeta)]
struct MinimumValidator {
    value: f64,
    #[darling(default)]
    exclusive: bool,
}

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
    #[darling(default)]
    default: Option<DefaultValue>,

    #[darling(default)]
    multiple_of: Option<SpannedValue<f64>>,
    #[darling(default)]
    maximum: Option<MaximumValidator>,
    #[darling(default)]
    minimum: Option<MinimumValidator>,
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
            fields.push(field_ident);
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

        let validators = create_validators(&crate_name, field)?;

        let validators_check = if !validators.is_empty() {
            Some(quote! {
                for validator in [#(#validators),*] {
                    if !#crate_name::validation::Validator::check(&validator, &value) {
                        return Err(#crate_name::types::ParseError::<Self>::custom(format!("field `{}` verification failed. {}", #field_name, validator)));
                    }
                }
            })
        } else {
            None
        };

        let validators_meta = if !validators.is_empty() {
            Some(quote! {
                for validator in [#(#validators),*] {
                    #crate_name::validation::Validator::<#field_ty>::update_meta(&validator, &mut meta_validators);
                }
            })
        } else {
            None
        };

        match &field.default {
            Some(default_value) => {
                let default_value = match default_value {
                    DefaultValue::Default => {
                        quote!(<#field_ty as ::std::default::Default>::default())
                    }
                    DefaultValue::Function(func_name) => quote!(#func_name()),
                };

                deserialize_fields.push(quote! {
                    #[allow(non_snake_case)]
                    let #field_ident: #field_ty = {
                        match obj.get(#field_name).cloned().unwrap_or_default() {
                            #crate_name::serde_json::Null => #default_value,
                            value => {
                                let value = #crate_name::types::Type::parse(value).map_err(#crate_name::types::ParseError::propagate)?;
                                #validators_check
                                value
                            }
                        }
                    };
                });
            }
            _ => {
                deserialize_fields.push(quote! {
                    #[allow(non_snake_case)]
                    let #field_ident: #field_ty = {
                        let value = #crate_name::types::Type::parse(obj.get(#field_name).cloned().unwrap_or_default())
                            .map_err(#crate_name::types::ParseError::propagate)?;
                        #validators_check
                        value
                    };
                });
            }
        };

        serialize_fields.push(quote! {
            let value = #crate_name::types::Type::to_value(&self.#field_ident);
            object.insert(::std::string::ToString::to_string(#field_name), value);
        });

        schema_fields.push(quote! {{
            <#field_ty>::register(registry);
            #[allow(unused_mut)]
            let mut meta_validators = <#crate_name::registry::MetaValidators as ::std::default::Default>::default();

            #validators_meta

            let property = #crate_name::registry::MetaProperty {
                data_type: <#field_ty>::DATA_TYPE,
                title: #field_title,
                description: #field_description,
                default: ::std::option::Option::None,
                validators: meta_validators,
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
                data_type: #crate_name::types::DataType::OBJECT,
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
                const DATA_TYPE: #crate_name::types::DataType = #crate_name::types::DataType::SchemaReference(#oai_typename);

                fn parse(value: #crate_name::serde_json::Value) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
                    if let #crate_name::serde_json::Value::Object(obj) = value {
                        #(#deserialize_fields)*
                        ::std::result::Result::Ok(Self { #(#fields),* })
                    } else {
                        ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value))
                    }
                }

                fn parse_from_str(_value: ::std::option::Option<&str>) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
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
                    #crate_name::types::Type::parse(value).map_err(|err| #crate_name::serde::de::Error::custom(err.into_message()))
                }
            }
        }
    } else {
        let mut code = Vec::new();

        code.push(quote! {
            impl #impl_generics #ident #ty_generics #where_clause {
                fn __internal_register(registry: &mut #crate_name::registry::Registry) where Self: #crate_name::Schema {
                    #register_type
                }

                fn __internal_parse(value: #crate_name::serde_json::Value) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>>  where Self: #crate_name::Schema {
                    if let #crate_name::serde_json::Value::Object(obj) = value {
                        #(#deserialize_fields)*
                        ::std::result::Result::Ok(Self { #(#fields),* })
                    } else {
                        ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value))
                    }
                }

                fn __internal_to_value(&self) -> #crate_name::serde_json::Value  where Self: #crate_name::Schema {
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
                    const DATA_TYPE: #crate_name::types::DataType = #crate_name::types::DataType::SchemaReference(#oai_typename);

                    fn parse(value: #crate_name::serde_json::Value) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
                        Self::__internal_parse(value)
                    }

                    fn parse_from_str(_value: ::std::option::Option<&str>) -> ::std::result::Result<Self, #crate_name::types::ParseError<Self>> {
                        ::std::result::Result::Err(#crate_name::types::ParseError::not_support_parsing_from_string())
                    }

                    fn to_value(&self) -> #crate_name::serde_json::Value {
                        Self::__internal_to_value(self)
                    }

                    fn register(registry: &mut #crate_name::registry::Registry) {
                        Self::__internal_register(registry);
                    }
                }

                impl #crate_name::Schema for #concrete_type {
                    const NAME: &'static str = #oai_typename;
                }
            };
            code.push(expanded);
        }

        quote!(#(#code)*)
    };

    Ok(expanded.into())
}

fn create_validators(
    crate_name: &TokenStream,
    field: &SchemaField,
) -> GeneratorResult<Vec<TokenStream>> {
    let mut validators = Vec::new();

    if let Some(value) = &field.multiple_of {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.1
        if &**value <= &0.0 {
            return Err(Error::new(
                value.span(),
                "The value of `multipleOf` MUST be a number, strictly greater than 0.",
            )
            .into());
        }
        let value = &**value;
        validators.push(quote!(#crate_name::validation::MultipleOf::new(#value)));
    }

    if let Some(MaximumValidator { value, exclusive }) = &field.maximum {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.2
        validators.push(quote!(#crate_name::validation::Maximum::new(#value, #exclusive)));
    }

    if let Some(MinimumValidator { value, exclusive }) = &field.minimum {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.4
        validators.push(quote!(#crate_name::validation::Minimum::new(#value, #exclusive)));
    }

    if let Some(value) = &field.max_length {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.6
        if &**value < &0 {
            return Err(Error::new(
                value.span(),
                "The value of `maxLength` MUST be an integer. This integer MUST be greater than, or equal to, 0.",
            )
                .into());
        }
        let value = &**value;
        validators.push(quote!(#crate_name::validation::MaxLength::new(#value)));
    }

    if let Some(value) = &field.min_length {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.7
        if &**value < &0 {
            return Err(Error::new(
                value.span(),
                "The value of `minLength` MUST be an integer. This integer MUST be greater than, or equal to, 0.",
            )
                .into());
        }
        let value = &**value;
        validators.push(quote!(#crate_name::validation::MinLength::new(#value)));
    }

    if let Some(value) = &field.pattern {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.8
        if let Err(err) = Regex::new(&**value) {
            return Err(
                Error::new(value.span(), format!("Invalid regular expression. {}", err)).into(),
            );
        }
        let value = &**value;
        validators.push(quote!(#crate_name::validation::Pattern::new(#value)));
    }

    if let Some(value) = &field.max_items {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.10
        if &**value < &0 {
            return Err(Error::new(
                value.span(),
                "The value of `maxItems` MUST be an integer. This integer MUST be greater than, or equal to, 0.",
            )
                .into());
        }
        let value = &**value;
        validators.push(quote!(#crate_name::validation::MaxItems::new(#value)));
    }

    if let Some(value) = &field.min_items {
        // https://datatracker.ietf.org/doc/html/draft-wright-json-schema-validation-00#section-5.11
        if &**value < &0 {
            return Err(Error::new(
                value.span(),
                "The value of `minItems` MUST be an integer. This integer MUST be greater than, or equal to, 0.",
            )
                .into());
        }
        let value = &**value;
        validators.push(quote!(#crate_name::validation::MinItems::new(#value)));
    }

    Ok(validators)
}
