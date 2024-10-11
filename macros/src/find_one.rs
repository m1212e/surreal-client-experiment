use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::quote;
use surreal_client_core::{
    query::{
        db_wrapper::DBWrapper,
        query::Query,
        result::{result_field::QueryResultField, result_struct::QueryResultStruct},
    },
    table::GenericTable,
};
use syn::Ident;

pub fn find_one_internal(input: syn::Item) -> proc_macro2::TokenStream {
    let table = GenericTable::from(input);

    let query = Query::new(table.name, table.fields.clone());
    let query_string_wrapped_with_db_call = DBWrapper::new(query, "db".to_string());
    let result_struct = QueryResultStruct::new(
        table
            .fields
            .clone()
            .into_iter()
            .map(QueryResultField::from)
            .collect::<Vec<_>>(),
    );
    let result_struct_name = syn::TypePath {
        qself: None,
        path: syn::parse_str(&result_struct.name).expect("Failed to parse result struct name"),
    };
    let result_variable_name =
        Ident::new(&result_struct.name.to_case(Case::Snake), Span::call_site());
    let result_variable_name_typed = Ident::new(
        &format!("{}Typed", result_variable_name).to_case(Case::Snake),
        Span::call_site(),
    );

    quote! {
        {
           #result_struct

           let mut #result_variable_name = #query_string_wrapped_with_db_call.await?;

           let #result_variable_name_typed: Option<#result_struct_name> = #result_variable_name.take(1)?;

           #result_variable_name_typed
       }
    }
}
