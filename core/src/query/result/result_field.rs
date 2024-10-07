use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use std::hash::Hash;
use syn::parse_str;
use syn::Ident;
use syn::Path;

use crate::field::Field;

#[derive(Hash, Debug)]
pub struct QueryResultField {
    field: Field,
}

impl From<Field> for QueryResultField {
    fn from(value: Field) -> Self {
        Self { field: value }
    }
}

impl ToTokens for QueryResultField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = Ident::new(&self.field.name, Span::call_site());
        let type_str = match &self.field.field_type {
            crate::field::FieldType::String => "String",
            crate::field::FieldType::Int => "i64",
            _ => todo!("Not implemented yet"),
        };
        let type_path = parse_str::<Path>(&type_str).expect("Invalid type path string");

        tokens.extend(quote! {
            #name: #type_path
        });
    }
}

impl QueryResultField {
    fn new(field: Field) -> Self {
        Self { field }
    }
}
