use quote::quote;
use surreal_client_core::field::Field;
use syn::DeriveInput;

pub fn table_internal(input: DeriveInput) -> proc_macro2::TokenStream {
    // Parse the input tokens into a syntax tree
    let name = &input.ident;

    let parsed_fields = match input.data.clone() {
        syn::Data::Struct(data_struct) => data_struct.fields.into_iter(),
        syn::Data::Enum(_data_enum) => panic!("Enums are not yet supported"),
        syn::Data::Union(_data_union) => panic!("Unions are not yet supported"),
    }
    .map(Field::from)
    .into_iter();

    quote! {
        #[surreal_client::macro_magic::export_tokens(#name)]
        #input

        impl surreal_client::Table for #name {
            fn name() -> String {
                stringify!(#name).to_string()
            }

            fn fields() -> Vec<surreal_client::Field> {
                vec![#(#parsed_fields),*]
            }
        }
    }
}
