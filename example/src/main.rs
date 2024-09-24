use surreal_client::{field::{Field, FieldType}, table::Table};
use surreal_client_macro::find_one;
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

fn main() {
    let query = find_one!(TestTable);
}
