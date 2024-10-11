use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Ident;

use super::result_field::QueryResultField;

#[derive(Hash, Debug)]
pub struct QueryResultStruct {
    fields: Vec<QueryResultField>,
    pub name: String,
}

impl ToTokens for QueryResultStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let struct_name = Ident::new(&self.name, Span::call_site());

        let fields = self.fields.iter();

        let r = quote! {
            #[derive(Hash, Debug, serde::Deserialize, serde::Serialize)]
            struct #struct_name {
                #(#fields),*
            }
        };
        tokens.extend(r);
    }
}

impl QueryResultStruct {
    pub fn new(fields: Vec<QueryResultField>) -> Self {
        let mut s = DefaultHasher::new();
        for field in &fields {
            field.hash(&mut s);
        }
        let mut hashed = s.finish().to_string();
        hashed.truncate(10);
        hashed = format!("QueryResult{}", hashed);

        Self {
            fields,
            name: hashed,
        }
    }
}
