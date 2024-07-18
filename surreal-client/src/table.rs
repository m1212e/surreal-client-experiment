use crate::{field::StaticField, query::base::Query};

pub trait Table<'a>: Sized {
    fn name() -> &'a str;
    fn fields() -> &'a [StaticField<'a>];
    fn find() -> Query<'a, Self>;
}
