use std::marker::PhantomData;
use crate::table::Table;

pub struct Query<'a, T: Table<'a>> {
    _phantom: PhantomData<&'a T>,
}

impl<'a, T: Table<'a>> Query<'a, T> {
    pub fn new() -> Self {
        Query {
            _phantom: PhantomData,
        }
    }

    pub fn build(&self) -> String {
        format!("SELECT * FROM {}", T::name())
    }
}
