use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parser, parse_macro_input, Visibility};

pub(crate) fn visibility_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let field_tokens = item.clone();
    let field_parser = syn::Field::parse_named;
    if let Ok(field) = field_parser.parse(field_tokens) {
        return quote!(#field).into();
    }

    let mut input = parse_macro_input!(item as syn::ItemStruct);
    let visibility = parse_macro_input!(args as syn::Visibility);

    input.fields.iter_mut().for_each(|f| {
        let field: &mut syn::Field = f;
        let default_vis = &visibility;
        let attr = field.attrs.iter().find(|attr| {
            attr.path().get_ident().map(syn::Ident::to_string) == Some(String::from("visibility"))
        });

        // Set field visibility
        if let Some(attr) = attr {
            if let Ok(vis) = attr.parse_args::<syn::Visibility>() {
                field.vis = vis;
            } else {
                field.vis = syn::Visibility::Inherited;
            }
        } else if !matches!(field.vis, Visibility::Inherited) {
            field.vis = default_vis.clone();
        }
    });
    quote!(#input).into()
}
