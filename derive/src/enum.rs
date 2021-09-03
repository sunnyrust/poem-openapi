use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromVariant,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, DeriveInput, Error};

use crate::{
    common_args::{RenameRule, RenameRuleExt, RenameTarget},
    error::GeneratorResult,
    utils::get_crate_name,
};

#[derive(FromVariant)]
#[darling(attributes(oai), forward_attrs(doc))]
struct EnumItem {
    ident: Ident,
    fields: Fields<Ignored>,

    #[darling(default)]
    pub name: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(oai), forward_attrs(doc))]
struct EnumArgs {
    ident: Ident,
    data: Data<EnumItem, Ignored>,

    #[darling(default)]
    internal: bool,
    #[darling(default)]
    rename_items: Option<RenameRule>,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let args: EnumArgs = EnumArgs::from_derive_input(&args)?;
    let crate_name = get_crate_name(args.internal);
    let ident = &args.ident;
    let e = match &args.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(ident, "Enum can only be applied to an enum.").into()),
    };
    let mut enum_items = Vec::new();
    let mut ident_to_item = Vec::new();
    let mut item_to_ident = Vec::new();

    for variant in e {
        if !variant.fields.is_empty() {
            return Err(Error::new_spanned(
                &variant.ident,
                format!(
                    "Invalid enum variant {}.\nOpenAPI enums may only contain unit variants.",
                    variant.ident
                ),
            )
            .into());
        }

        let item_ident = &variant.ident;
        let oai_item_name = variant.name.clone().unwrap_or_else(|| {
            args.rename_items
                .rename(variant.ident.unraw().to_string(), RenameTarget::EnumItem)
        });

        enum_items.push(quote!(#oai_item_name));
        ident_to_item.push(quote!(#ident::#item_ident => #oai_item_name));
        item_to_ident
            .push(quote!(#oai_item_name => ::std::result::Result::Ok(#ident::#item_ident)));
    }

    let expanded = quote! {
        impl #crate_name::types::Type for #ident {
            const DATA_TYPE: #crate_name::types::DataType = #crate_name::types::DataType::Enum {
                items: &[#(#enum_items),*],
            };

            fn parse(value: ::std::option::Option<#crate_name::serde_json::Value>) -> #crate_name::types::ParseResult<Self> {
                if let ::std::option::Option::Some(#crate_name::serde_json::Value::String(item)) = &value {
                    match item.as_str() {
                        #(#item_to_ident,)*
                        _ => ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value.unwrap_or_default())),
                    }
                } else {
                    ::std::result::Result::Err(#crate_name::types::ParseError::expected_type(value.unwrap_or_default()))
                }
            }

            fn parse_from_str(value: ::std::option::Option<&str>) -> #crate_name::types::ParseResult<Self> {
                match value {
                    ::std::option::Option::Some(value) => match value {
                        #(#item_to_ident,)*
                        _ => ::std::result::Result::Err(#crate_name::types::ParseError::custom("Expect a valid enumeration value.")),
                    },
                    ::std::option::Option::None => ::std::result::Result::Err(#crate_name::types::ParseError::expected_input()),
                }
            }

            fn to_value(&self) -> #crate_name::serde_json::Value {
                let name = match self {
                    #(#ident_to_item),*
                };
                #crate_name::serde_json::Value::String(::std::string::ToString::to_string(name))
            }
        }
    };

    Ok(expanded.into())
}
