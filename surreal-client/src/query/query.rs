use std::collections::HashMap;

use quote::ToTokens;

use crate::{field::Field, query::query_part::QueryBuilderPart, table::Table};

use super::{selection::Selection, table_specifier::TableSpecifier};

pub struct Query<'a> {
    selected_fields: Selection<'a>,
    table_specifier: TableSpecifier,
}

impl<'a> QueryBuilderPart for Query<'a> {
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

impl<'a> ToTokens for Query<'a> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let query_string = self.to_string();
        tokens.extend(quote::quote! { #query_string });
    }
}

impl<'a> Query<'a> {
    pub fn new<T: Table<'a>>(selected_fields: Vec<Field<'a>>) -> Self {
        let selected_fields = match selected_fields.is_empty() {
            true => Selection::new(T::fields()),
            false => Selection::new(selected_fields),
        };
        let table_specifier = TableSpecifier::new(T::name());

        Self {
            selected_fields,
            table_specifier,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::FieldType;

    use super::*;

    struct TestTable {}

    impl<'a> Table<'a> for TestTable {
        fn name() -> String {
            "test_table".to_string()
        }

        fn fields() -> Vec<Field<'a>> {
            vec![
                Field {
                    name: "a".to_string(),
                    field_type: FieldType::Any,
                },
                Field {
                    name: "b".to_string(),
                    field_type: FieldType::Any,
                },
            ]
        }
    }

    #[test]
    fn test_build() {
        let query = Query::new::<TestTable>(TestTable::fields());
        let built = query.to_string();
        assert_eq!(built, "SELECT $a, $b FROM type::table($table);");

        let bindings = query.bindings();
        assert_eq!(bindings.len(), 3);
        assert_eq!(bindings.get("a"), Some(&"a".to_string()));
        assert_eq!(bindings.get("b"), Some(&"b".to_string()));
        assert_eq!(bindings.get("table"), Some(&"test_table".to_string()));
    }

    #[test]
    fn test_build_one_field() {
        let query = Query::new::<TestTable>(vec![Field {
            name: "a".to_string(),
            field_type: FieldType::Any,
        }]);
        let built = query.to_string();
        assert_eq!(built, "SELECT $a FROM type::table($table);");

        let bindings = query.bindings();
        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings.get("a"), Some(&"a".to_string()));
        assert_eq!(bindings.get("table"), Some(&"test_table".to_string()));
    }

    #[test]
    fn test_build_no_field() {
        let query = Query::new::<TestTable>(vec![]);
        let built = query.to_string();
        assert_eq!(built, "SELECT $a, $b FROM type::table($table);");

        let bindings = query.bindings();
        assert_eq!(bindings.len(), 3);
        assert_eq!(bindings.get("a"), Some(&"a".to_string()));
        assert_eq!(bindings.get("b"), Some(&"b".to_string()));
        assert_eq!(bindings.get("table"), Some(&"test_table".to_string()));
    }
}
