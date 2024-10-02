use macro_state::proc_write_state;
use quote::quote;
use syn::DeriveInput;

pub fn table_internal(input: DeriveInput) -> proc_macro2::TokenStream {
    // Parse the input tokens into a syntax tree
    let name = input.ident;

    let parsed_fields = match input.data {
        syn::Data::Struct(data_struct) => data_struct.fields.into_iter(),
        syn::Data::Enum(_data_enum) => panic!("Enums are not yet supported"),
        syn::Data::Union(_data_union) => panic!("Unions are not yet supported"),
    }
    .map(surreal_client::field::Field::from)
    .into_iter();

    let serialized_table_fields = serde_json::to_string(&parsed_fields.clone().collect::<Vec<_>>())
        .expect("Failed to serialize table fields");

    proc_write_state(&name.to_string(), &serialized_table_fields).expect("Failed to write state");

    quote! {
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
