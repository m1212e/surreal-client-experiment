use std::collections::HashMap;

use quote::ToTokens;

use crate::{field::Field, query::query_part::QueryBuilderPart};

use super::{selection::Selection, table_specifier::TableSpecifier};

#[derive(Debug, Clone)]
pub struct Query {
    selected_fields: Selection,
    table_specifier: TableSpecifier,
}

impl QueryBuilderPart for Query {
    fn to_string(&self) -> String {
        format!(
            "{} {};",
            self.selected_fields.to_string(),
            self.table_specifier.to_string()
        )
    }

    fn bindings(&self) -> HashMap<String, String> {
        let mut map = self.selected_fields.bindings();
        map.extend(self.table_specifier.bindings());
        map
    }
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let query_string = self.to_string();
        tokens.extend(quote::quote! { #query_string });
    }
}

impl Query {
    pub fn new(table_specifier: String, selected_fields: Vec<Field>) -> Self {
        Self {
            selected_fields: Selection::new(selected_fields),
            table_specifier: TableSpecifier::new(table_specifier),
        }
    }
}
