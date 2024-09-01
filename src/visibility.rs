use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Visibility};

pub(crate) fn visibility_impl(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(item as syn::ItemStruct);
    let visibility = parse_macro_input!(args as syn::Visibility);

    input.fields.iter_mut().for_each(|f| {
        if matches!(f.vis, Visibility::Inherited) {
            f.vis = visibility.clone();
        }
    });
    quote!(#input).into()
}
