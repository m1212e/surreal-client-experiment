use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

#[derive(Debug, Hash, Clone)]
pub enum FieldType {
    Any,
    Array(Box<ManyFieldType>),
    Set(Box<ManyFieldType>),
    Bool,
    Bytes,
    DateTime,
    Decimal,
    Duration,
    Float,
    Int,
    Number,
    Object,
    Option(Box<FieldType>),
    String,
    Record,
}

impl ToTokens for FieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldType::Any => tokens.extend(quote! { surreal_client::FieldType::Any }),
            FieldType::Array(field) => {
                tokens.extend(quote! { surreal_client::FieldType::Array(#field) })
            }
            FieldType::Set(field) => {
                tokens.extend(quote! { surreal_client::FieldType::Set(#field) })
            }
            FieldType::Bool => tokens.extend(quote! { surreal_client::FieldType::Bool }),
            FieldType::Bytes => tokens.extend(quote! { surreal_client::FieldType::Bytes }),
            FieldType::DateTime => tokens.extend(quote! { surreal_client::FieldType::DateTime }),
            FieldType::Decimal => tokens.extend(quote! { surreal_client::FieldType::Decimal }),
            FieldType::Duration => tokens.extend(quote! { surreal_client::FieldType::Duration }),
            FieldType::Float => tokens.extend(quote! { surreal_client::FieldType::Float }),
            FieldType::Int => tokens.extend(quote! { surreal_client::FieldType::Int }),
            FieldType::Number => tokens.extend(quote! { surreal_client::FieldType::Number }),
            FieldType::Object => tokens.extend(quote! { surreal_client::FieldType::Object }),
            FieldType::Option(field) => {
                let inner = field.to_token_stream();
                tokens.extend(quote! { surreal_client::FieldType::Option(&#inner) })
            }
            FieldType::String => tokens.extend(quote! { surreal_client::FieldType::String }),
            FieldType::Record => tokens.extend(quote! { surreal_client::FieldType::Record }),
        };
    }
}

#[derive(Debug, Hash, Clone)]
pub struct ManyFieldType {
    pub field_type: FieldType,
    pub max_length: Option<usize>,
}

impl<'a> ToTokens for ManyFieldType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_type = &self.field_type;
        let max_length = self.max_length;

        tokens.extend(quote! {
            surreal_client::field::ManyFieldType {
                field_type: #field_type,
                max_length: #max_length,
            }
        });
    }
}

#[derive(Debug, Hash, Clone)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let field_type = &self.field_type;

        tokens.extend(quote! {
            surreal_client::Field {
                name: #name.to_string(),
                field_type: #field_type,
            }
        });
    }
}

impl From<&syn::Field> for Field {
    fn from(value: &syn::Field) -> Self {
        let name = value
            .ident
            .as_ref()
            .expect("Field must have a name")
            .to_string();

        let value_type = &value.ty;
        let stringified_field_type = quote!(#value_type).to_string();
        let invalid_field_type_error_message = format!("Invalid field type '{}' on field '{}'", stringified_field_type, name);

        //TODO: Implement field type parsing
        let field_type = match &value.ty {
            syn::Type::Path(type_path) => {
                if type_path.path.segments.len() != 1
                    || !type_path.path.segments[0].arguments.is_none()
                {
                    panic!("{}", invalid_field_type_error_message)
                }

                let ident = &type_path.path.segments[0].ident;

                match ident.to_string().as_str() {
                    "String" => FieldType::String,
                    "i8" | "u8" | "i16" | "u16" | "i32" | "u32" | "u64" | "i128" | "u128" => {
                        panic!(
                            "{}, only 'i64' integers are supported!",
                            invalid_field_type_error_message
                        )
                    }
                    "i64" => FieldType::Int,
                    _ => panic!("{}", invalid_field_type_error_message),
                }
            }
            _ => panic!("{}", invalid_field_type_error_message),
        };

        Field {
            name: name,
            field_type: field_type,
        }
    }
}

impl From<syn::Field> for Field {
    fn from(value: syn::Field) -> Self {
        Self::from(&value)
    }
}
