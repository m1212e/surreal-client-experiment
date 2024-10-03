use std::collections::HashMap;

use super::query_part::QueryBuilderPart;
use crate::field::Field;

pub struct Selection {
    selected_fields: Vec<Field>,
}

impl QueryBuilderPart for Selection {
    fn to_string(&self) -> String {
        format!(
            "SELECT {}",
            self.selected_fields
                .iter()
                .map(|f| format!("${}", f.name))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }

    fn bindings(&self) -> HashMap<String, String> {
        self.selected_fields
            .iter()
            .map(|f| (f.name.to_string(), f.name.to_string()))
            .collect()
    }
}

impl Selection {
    pub fn new(selected_fields: Vec<Field>) -> Self {
        Selection { selected_fields }
    }
}

#[cfg(test)]
mod tests {
    use crate::field::FieldType;

    use super::*;

    #[test]
    fn test_to_string() {
        let input = vec![
            Field {
                name: "a".to_string(),
                field_type: FieldType::Any,
            },
            Field {
                name: "b".to_string(),
                field_type: FieldType::Any,
            },
        ];
        let selection = Selection::new(input);
        assert_eq!(selection.to_string(), "SELECT $a, $b");
    }

    #[test]
    fn test_bindings() {
        let input = vec![
            Field {
                name: "a".to_string(),
                field_type: FieldType::Any,
            },
            Field {
                name: "b".to_string(),
                field_type: FieldType::Any,
            },
        ];
        let selection = Selection::new(input);
        let bindings = selection.bindings();
        assert_eq!(bindings.len(), 2);
        assert_eq!(bindings.get("a"), Some(&"a".to_string()));
        assert_eq!(bindings.get("b"), Some(&"b".to_string()));
    }
}
