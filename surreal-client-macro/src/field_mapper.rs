use surreal_client::field::FieldType;

pub fn map_syn_field_to_surreal_client_field_token_stream(input: &syn::Field) -> TokenStream {
    let field_type = match &input.ty {
        syn::Type::Path(type_path) => {
            match type_path
                .path
                .get_ident()
                .expect("Unsupported type (ident)")
                .to_string()
                .as_str()
            {
                "String" => FieldType::String,
                _ => panic!("Unsupported type (ident)"),
            }
        }
        syn::Type::Array(type_array) => FieldType::Array(
            map_syn_field_to_surreal_client_field_token_stream(&type_array.elem),
        ),
        _ => panic!("Unsupported type"),
    };

    // we leave this here to cause compile breaks in case of a change
    // please make sure that the quote output is in sync!
    // let _typecheck = surreal_client::field::Field {
    //     name:
    // }

    quote! {
        surreal_client::field::Field {

        }
    }
}


fn syn_field_to_surreal_client_field(input: &syn::Field) -> FieldType {
    match &input.ty {
        syn::Type::Path(type_path) => {
            match type_path
                .path
                .get_ident()
                .expect("Unsupported type (ident)")
                .to_string()
                .as_str()
            {
                "String" => FieldType::String,
                _ => panic!("Unsupported type (ident)"),
            }
        }
        syn::Type::Array(type_array) => FieldType::Array(
            map_syn_field_to_surreal_client_field_token_stream(&type_array.elem),
        ),
        _ => panic!("Unsupported type"),
    }
}