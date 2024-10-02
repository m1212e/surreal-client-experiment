use crate::field::Field;

pub trait Table {
    fn name() -> String;
    fn fields() -> Vec<Field>;
}
