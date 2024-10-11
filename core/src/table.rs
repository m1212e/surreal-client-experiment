use crate::field::Field;
pub trait Table {
    fn name() -> String;
    fn fields() -> Vec<Field>;
}

pub struct GenericTable {
    pub name: String,
    pub fields: Vec<Field>,
}

impl From<syn::Item> for GenericTable {
    fn from(value: syn::Item) -> Self {
        let element_name = match &value {
            syn::Item::Struct(item_struct) => item_struct.ident.to_string(),
            syn::Item::Enum(_item_enum) => panic!("Enums are not yet supported!"),
            _ => panic!("Invalid type"),
        };

        let element_fields = match value {
            syn::Item::Struct(item_struct) => item_struct.fields,
            syn::Item::Enum(_item_enum) => panic!("Enums are not yet supported!"),
            _ => panic!("Invalid type"),
        };
    
        let converted_fields = element_fields
            .into_iter()
            .map(Field::from)
            .collect::<Vec<_>>();

        Self {
            name: element_name,
            fields: converted_fields
        }
    }
}