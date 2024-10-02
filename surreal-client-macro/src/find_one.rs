use macro_state::proc_read_state;
use quote::quote;
use surreal_client::query::result::TargetStruct;

pub fn find_one_internal(input: syn::Type) -> proc_macro2::TokenStream {
    let path = match input {
        syn::Type::Path(path) => path.path,
        _ => panic!("Invalid type"),
    };

    let table_name = path
        .segments
        .last()
        .expect("Invalid path")
        .ident
        .to_string();

    let raw_table_fields = proc_read_state(&table_name).expect("Failed to read state");
    let table_fields = serde_json::from_str::<Vec<surreal_client::field::Field>>(&raw_table_fields)
        .expect("Failed to deserialize table fields");

    let result = TargetStruct::new(table_fields);

    quote! {#result}
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
