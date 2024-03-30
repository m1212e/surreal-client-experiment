pub enum FieldType<'a> {
    Any,
    Array(&'a ManyFieldType<'a>),
    Set(&'a ManyFieldType<'a>),
    Bool,
    Bytes,
    DateTime,
    Decimal,
    Duration,
    Float,
    Int,
    Number,
    Object,
    Option(&'a FieldType<'a>),
    String,
    Record
}

pub struct ManyFieldType<'a> {
    pub field_type: FieldType<'a>,
    pub max_length: Option<usize>,
}

pub struct Field<'a> {
    pub name: &'a str,
    pub field_type: &'a FieldType<'a>,
}
