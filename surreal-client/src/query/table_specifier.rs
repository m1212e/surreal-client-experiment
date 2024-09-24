use std::collections::HashMap;

use super::query_part::QueryBuilderPart;

pub struct TableSpecifier {
    table_name: String,
}

impl QueryBuilderPart for TableSpecifier {
    fn to_string(&self) -> String {
        "FROM type::table($table)".to_string()
    }

    fn bindings(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("table".to_string(), self.table_name.to_string());
        map
    }
}

impl TableSpecifier {
    pub fn new(table_name: String) -> Self {
        TableSpecifier { table_name }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let table_name = "my_table";
        let table_specifier = TableSpecifier::new(table_name.to_string());
        assert_eq!(table_specifier.to_string(), "FROM type::table($table)");
    }

    #[test]
    fn test_bindings() {
        let table_name = "my_table";
        let table_specifier = TableSpecifier::new(table_name.to_string());
        let bindings = table_specifier.bindings();
        assert_eq!(bindings.len(), 1);
        assert_eq!(bindings.get("table"), Some(&"my_table".to_string()));
    }
}
