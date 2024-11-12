use crate::field::Field;

pub mod table_select;

pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

impl From<&syn::Item> for Table {
    fn from(value: &syn::Item) -> Self {
        let name = match &value {
            syn::Item::Struct(item_struct) => item_struct.ident.to_string(),
            syn::Item::Enum(_item_enum) => panic!("Enums are not yet supported!"),
            _ => panic!("Invalid type"),
        };

        let fields = match value {
            syn::Item::Struct(item_struct) => item_struct
                .fields
                .iter()
                .map(Field::from)
                .collect::<Vec<_>>(),
            syn::Item::Enum(_item_enum) => panic!("Enums are not yet supported!"),
            _ => panic!("Invalid type"),
        };

        Self { name, fields }
    }
}

impl From<syn::Item> for Table {
    fn from(value: syn::Item) -> Self {
        Table::from(&value)
    }
}

impl From<&syn::DeriveInput> for Table {
    fn from(value: &syn::DeriveInput) -> Self {
        let name = value.ident.to_string();
        let fields = match &value.data {
            syn::Data::Struct(data_struct) => {
                data_struct.fields.iter().map(Field::from).collect()
            }
            syn::Data::Enum(_data_enum) => panic!("Enums are not yet supported!"),
            syn::Data::Union(_data_union) => panic!("Unions are not yet supported!"),
        };
        Self { name, fields }
    }
}

impl From<syn::DeriveInput> for Table {
    fn from(value: syn::DeriveInput) -> Self {
        Table::from(&value)
    }
}