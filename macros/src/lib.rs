use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use find_one::find_one_internal;
use table::table_internal;

mod find_one;
mod table;

#[macro_magic::import_tokens_proc(surreal_client)]
#[proc_macro]
pub fn find_one(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Item);
    find_one_internal(input).into()
}

#[proc_macro_attribute]
pub fn table(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    table_internal(item).into()
}


// macro magic re exports

// #[proc_macro_attribute]
// pub fn export_tokens(attr: TokenStream, tokens: TokenStream) -> TokenStream {
//     match macro_magic::mm_core::export_tokens_internal(attr, tokens, true, true) {
//         Ok(tokens) => tokens.into(),
//         Err(err) => err.to_compile_error().into(),
//     }
// }

// #[proc_macro]
// pub fn forward_tokens(tokens: TokenStream) -> TokenStream {
//     match macro_magic::mm_core::forward_tokens_internal(tokens, true) {
//         Ok(tokens) => tokens.into(),
//         Err(err) => err.to_compile_error().into(),
//     }
// }