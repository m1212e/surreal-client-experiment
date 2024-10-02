use syn::{parse_macro_input, DeriveInput};
use proc_macro::TokenStream;

use table::table_internal;
use find_one::find_one_internal;

mod table;
mod find_one;

#[proc_macro]
pub fn find_one(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Type);
    find_one_internal(input).into()
}


#[proc_macro_derive(Table)]
pub fn table(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    table_internal(input).into()
}
