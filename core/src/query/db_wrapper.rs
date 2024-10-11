use proc_macro2::Span;
use quote::{quote, ToTokens};
use syn::Ident;

use super::{query::Query, query_part::QueryBuilderPart};

pub struct DBWrapper {
    query: Query,
    db_variable: String,
}

impl ToTokens for DBWrapper {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let query_literal = &self.query;
        let db_variable = Ident::new(&self.db_variable, Span::call_site());
        let bindings = self.query.bindings();

        tokens.extend(quote! { #db_variable.query(#query_literal) });

        for (key, value) in bindings.iter() {
            tokens.extend(quote! { .bind((#key,#value)) });
        }
    }
}

impl DBWrapper {
    pub fn new(query: Query, db_variable: String) -> Self {
        Self {
            query,
            db_variable,
        }
    }
}