use crate::field::Field;

use super::query::Query;

pub trait Table<'a>: Sized {
    fn name() -> &'a str;
    fn fields() -> &'a [Field<'a>];
    fn find() -> Query<'a, Self>;
}
