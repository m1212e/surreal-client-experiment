use std::hash::DefaultHasher;
use std::hash::Hash;
use std::hash::Hasher;

use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn::Ident;

use crate::field::Field;

#[derive(Hash)]
pub struct TargetStruct {
    fields: Vec<Field>,
}

impl ToTokens for TargetStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut s = DefaultHasher::new();
        for field in &self.fields {
            field.hash(&mut s);
        }
        let mut hashed = s.finish().to_string();
        hashed.truncate(10);
        hashed = format!("query_result_{}", hashed);
        let struct_name = Ident::new(&hashed, Span::call_site());

        let fields = self.fields.iter().map(|f| f.name.clone());

        // #(#fields),*
        let r = quote! {
            {
                struct #struct_name {
                    name: String,
                }

                #struct_name {
                    name: String::from("daawwad"),
                }
            }
        };
        tokens.extend(r);
    }
}

impl TargetStruct {
    pub fn new(fields: Vec<Field>) -> Self {
        Self { fields }
    }
}
