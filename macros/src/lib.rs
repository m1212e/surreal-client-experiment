use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

use find_one::find_one_internal;
use table::table_internal;

mod find_one;
mod table;

#[macro_magic::import_tokens_proc(surreal_client::macro_magic)]
#[proc_macro]
pub fn find_one(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Item);
    find_one_internal(input).into()
}

#[proc_macro_attribute]
pub fn table(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as DeriveInput);
    table_internal(item).into()
}
