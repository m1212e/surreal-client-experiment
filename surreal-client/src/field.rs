use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Debug, Hash)]
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
            FieldType::Any => tokens.extend(quote! { surreal_client::field::FieldType::Any }),
            FieldType::Array(field) => {
                tokens.extend(quote! { surreal_client::field::FieldType::Array(#field) })
            }
            FieldType::Set(field) => {
                tokens.extend(quote! { surreal_client::field::FieldType::Set(#field) })
            }
            FieldType::Bool => tokens.extend(quote! { surreal_client::field::FieldType::Bool }),
            FieldType::Bytes => tokens.extend(quote! { surreal_client::field::FieldType::Bytes }),
            FieldType::DateTime => {
                tokens.extend(quote! { surreal_client::field::FieldType::DateTime })
            }
            FieldType::Decimal => {
                tokens.extend(quote! { surreal_client::field::FieldType::Decimal })
            }
            FieldType::Duration => {
                tokens.extend(quote! { surreal_client::field::FieldType::Duration })
            }
            FieldType::Float => tokens.extend(quote! { surreal_client::field::FieldType::Float }),
            FieldType::Int => tokens.extend(quote! { surreal_client::field::FieldType::Int }),
            FieldType::Number => tokens.extend(quote! { surreal_client::field::FieldType::Number }),
            FieldType::Object => tokens.extend(quote! { surreal_client::field::FieldType::Object }),
            FieldType::Option(field) => {
                let inner = field.to_token_stream();
                tokens.extend(quote! { surreal_client::field::FieldType::Option(&#inner) })
            }
            FieldType::String => tokens.extend(quote! { surreal_client::field::FieldType::String }),
            FieldType::Record => tokens.extend(quote! { surreal_client::field::FieldType::Record }),
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Hash)]
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

#[derive(Serialize, Deserialize, Debug, Hash)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let field_type = &self.field_type;

        tokens.extend(quote! {
            surreal_client::field::Field {
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

        //TODO: Implement field type parsing
        let field_type = FieldType::Any;

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
