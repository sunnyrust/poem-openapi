use darling::{
    ast::{Data, Fields},
    util::Ignored,
    FromDeriveInput, FromVariant,
};
use proc_macro2::{Ident, TokenStream};
use quote::quote;
use syn::{ext::IdentExt, Attribute, DeriveInput, Error};

use crate::{
    common_args::{RenameRule, RenameRuleExt, RenameTarget},
    error::GeneratorResult,
    utils::{get_crate_name, get_description, optional_literal},
};

#[derive(FromVariant)]
#[darling(attributes(oai), forward_attrs(doc))]
struct TagItem {
    ident: Ident,
    fields: Fields<Ignored>,
    attrs: Vec<Attribute>,

    #[darling(default)]
    pub name: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(oai), forward_attrs(doc))]
struct TagsArgs {
    ident: Ident,
    data: Data<TagItem, Ignored>,

    #[darling(default)]
    internal: bool,
    #[darling(default)]
    rename_items: Option<RenameRule>,
}

pub(crate) fn generate(args: DeriveInput) -> GeneratorResult<TokenStream> {
    let args: TagsArgs = TagsArgs::from_derive_input(&args)?;
    let crate_name = get_crate_name(args.internal);
    let ident = &args.ident;

    let e = match &args.data {
        Data::Enum(e) => e,
        _ => return Err(Error::new_spanned(ident, "Tags can only be applied to an enum.").into()),
    };

    let mut meta_items = Vec::new();
    let mut to_names = Vec::new();

    for variant in e {
        if !variant.fields.is_empty() {
            return Err(Error::new_spanned(
                &variant.ident,
                format!(
                    "Invalid enum variant {}.\nOpenAPI tags may only contain unit variants.",
                    variant.ident
                ),
            )
            .into());
        }

        let item_ident = &variant.ident;
        let oai_item_name = variant.name.clone().unwrap_or_else(|| {
            args.rename_items
                .rename(variant.ident.unraw().to_string(), RenameTarget::Tag)
        });
        let description = get_description(&variant.attrs)?;
        let description = optional_literal(&description);

        meta_items.push(quote!(#crate_name::registry::MetaTag {
            name: #oai_item_name,
            description: #description,
        }));
        to_names.push(quote!(Self::#item_ident => #oai_item_name));
    }

    let expanded = quote! {
        impl #crate_name::Tags for #ident {
            fn register(&self, registry: &mut #crate_name::registry::Registry) {
                #(registry.create_tag(#meta_items);)*
            }

            fn name(&self) -> &'static str {
                match self {
                #(#to_names),*
                }
            }
        }
    };

    Ok(expanded)
}
