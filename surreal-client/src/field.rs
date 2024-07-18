use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

pub enum FieldType<'a> {
    Any,
    Array(&'a ManyFieldType<'a>),
    Set(&'a ManyFieldType<'a>),
    Bool,
    Bytes,
    DateTime,
    Decimal,
    Duration,
    Float,
    Int,
    Number,
    Object,
    Option(&'a FieldType<'a>),
    String,
    Record,
}

impl<'a> ToTokens for FieldType<'a> {
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

pub struct ManyFieldType<'a> {
    pub field_type: FieldType<'a>,
    pub max_length: Option<usize>,
}

impl<'a> ToTokens for ManyFieldType<'a> {
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

pub struct Field<'a> {
    pub name: String,
    pub field_type: FieldType<'a>,
}

pub struct StaticField<'a> {
    pub name: &'a str,
    pub field_type: &'a FieldType<'a>,
}

impl<'a> ToTokens for StaticField<'a> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let field_type = &self.field_type;

        tokens.extend(quote! {
            surreal_client::field::StaticField {
                name: #name,
                field_type: &#field_type,
            }
        });
    }
}

impl<'a> From<&syn::Field> for Field<'a> {
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

impl<'a> From<&'a Field<'a>> for StaticField<'a> {
    fn from(value: &'a Field<'a>) -> Self {
        StaticField {
            name: &value.name,
            field_type: &value.field_type,
        }
    }
}
