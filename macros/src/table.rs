use proc_macro2::Span;
use quote::quote;
use surreal_client_core::table::{table_select::TableSelect, Table};
use syn::{DeriveInput, Ident};

pub fn table_internal(item: DeriveInput) -> proc_macro2::TokenStream {
    let table = Table::from(&item);
    let name = Ident::new(&table.name, Span::call_site());
    let select = TableSelect::new(&table);
    

    quote! {
        #[surreal_client::macro_magic::export_tokens(#name)]
        #item

        #select

    }
}
