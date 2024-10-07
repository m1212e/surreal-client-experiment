use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use result_field::QueryResultField;
use result_struct::QueryResultStruct;

use crate::field::Field;

mod result_field;
mod result_struct;

#[derive(Debug)]
pub struct QueryResult {
    result_struct: QueryResultStruct,
}

impl ToTokens for QueryResult {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let s = &self.result_struct;
        let r = quote! { 
            {
                #s

                "adawdawd"
            }
         };
        tokens.extend(r);
    }
}

impl QueryResult {
    pub fn new(fields: Vec<Field>) -> Self {
        let result_struct =
            QueryResultStruct::new(fields.into_iter().map(QueryResultField::from).collect());

        Self { result_struct }
    }
}
