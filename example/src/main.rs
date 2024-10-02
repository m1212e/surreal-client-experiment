use surreal_client_macro::find_one;
use surreal_client_macro::Table;

#[derive(Table)]
struct User {
    name: String,
    age: i32,
}

#[derive(Table)]
struct Person {
    name: String,
    age: i32,
}

#[derive(Table)]
struct Human {
    name: String,
    age: i32,
}

fn main() {
    let query = find_one!(User);
}
