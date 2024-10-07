use quote::quote;
use surreal_client_core::query::result::QueryResult;

pub fn find_one_internal(input: syn::Item) -> proc_macro2::TokenStream {
    let element_fields = match input {
        syn::Item::Struct(item_struct) => item_struct.fields,
        syn::Item::Enum(item_enum) => panic!("Enums are not yet supported!"),
        _ => panic!("Invalid type"),
    };

    let converted_fields = element_fields
        .into_iter()
        .map(surreal_client_core::field::Field::from)
        .collect::<Vec<_>>();

    
    let result = QueryResult::new(converted_fields);

    quote! { #result }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn example_macro_generates_correct_token_stream() {
//         let input = syn::parse_str::<syn::Type>("TestTable").unwrap();
//         let output = find_one_internal(input);
//         let expected_output = quote! {
//            surreal_client::query::query::Query::new::<TestTable>(TestTable::fields())
//         };
//         assert_eq!(output.to_string(), expected_output.to_string());
//     }
// }
