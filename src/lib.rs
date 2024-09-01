mod visibility;

use proc_macro::TokenStream;
use visibility::visibility_impl;

/// Attribute to set the visibility of all fields in a `struct`
#[proc_macro_attribute]
pub fn visibility(args: TokenStream, item: TokenStream) -> TokenStream {
    visibility_impl(args, item)
}
