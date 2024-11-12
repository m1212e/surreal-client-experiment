use proc_macro2::Span;
use quote::quote;
use quote::ToTokens;
use syn::Ident;
use convert_case::{Case, Casing};

use super::Table;

pub struct TableSelect<'a> {
    table: &'a Table,
}

impl<'a> TableSelect<'a> {
    pub fn new(table: &'a Table) -> Self {
        Self { table }
    }
}

impl<'a> ToTokens for TableSelect<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let table = &self.table;
        let select_enum_name = Ident::new(&format!("{}Select", &table.name), Span::call_site());
        let select_enum_variants = table
            .fields
            .iter()
            .map(|field| Ident::new(&field.name.to_case(Case::UpperCamel), Span::call_site()));

        tokens.extend(quote! {
            enum #select_enum_name {
                #(#select_enum_variants,)*
            }
        });
    }
}
