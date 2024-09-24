use std::collections::HashMap;

pub trait QueryBuilderPart {
    fn to_string(&self) -> String;
    fn bindings(&self) -> HashMap<String, String>;
}
