use quote::quote;
use syn::{token::Struct, DeriveInput};

pub fn table_internal(input: DeriveInput) -> proc_macro2::TokenStream {
    // Parse the input tokens into a syntax tree
    let name = &input.ident;

    let parsed_fields = match input.data.clone() {
        syn::Data::Struct(data_struct) => data_struct.fields.into_iter(),
        syn::Data::Enum(_data_enum) => panic!("Enums are not yet supported"),
        syn::Data::Union(_data_union) => panic!("Unions are not yet supported"),
    }
    .map(surreal_client::field::Field::from)
    .into_iter();

    let serialized_table_fields = serde_json::to_string(&parsed_fields.clone().collect::<Vec<_>>())
        .expect("Failed to serialize table fields");

    quote! {
        #[surreal_client_macro::export_tokens(#name)]
        #input

        impl surreal_client::table::Table for #name {
            fn name() -> String {
                stringify!(#name).to_string()
            }

            fn fields() -> Vec<surreal_client::field::Field> {
                vec![#(#parsed_fields),*]
            }
        }
    }
}
