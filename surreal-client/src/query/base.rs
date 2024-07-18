use std::marker::PhantomData;

use crate::table::Table;

pub struct Query<'a, T: Table<'a>> {
    _phantom: PhantomData<&'a T>,
    selected_fields: Vec<&'a T::>,
}

impl<'a, T: Table<'a>> Query<'a, T> {
    pub fn new() -> Self {
        Query {
            _phantom: PhantomData,
            selected_fields: Vec::new(),
        }
    }

    pub fn build(&self) -> String {
        format!(
            "SELECT {} FROM {}",
            T::fields()
                .iter()
                .map(|f| f.name)
                .collect::<Vec<_>>()
                .join(", "),
            T::name()
        )
    }
}
