extern crate proc_macro;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn find_one(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as syn::Type);
    find_one_internal(input).into()
}

fn find_one_internal(input: syn::Type) -> proc_macro2::TokenStream {
    quote! { 
        surreal_client::query::query::Query::new::<#input>(#input::fields())
     }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_macro_generates_correct_token_stream() {
        let input = syn::parse_str::<syn::Type>("TestTable").unwrap();
        let output = find_one_internal(input);
        let expected_output = quote! { 
            surreal_client::query::query::Query::new::<TestTable>(TestTable::fields())
         };
        assert_eq!(output.to_string(), expected_output.to_string());
    }
}
