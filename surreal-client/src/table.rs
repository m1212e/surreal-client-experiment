use crate::field::Field;

pub trait Table<'a> {
    fn name() -> String;
    fn fields() -> Vec<Field<'a>>;
}
