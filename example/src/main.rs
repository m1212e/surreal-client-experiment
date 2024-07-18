use surreal_client::table::Table;
use surreal_client_macro::Table;

#[derive(Table)]
struct Person {
    username: String,
    first_name: String,
    last_name: String,
}

#[derive(Table)]
struct Post {
    title: String,
}

fn main() {
    let person = Person {
        username: "xXJohn_DoeXx".to_string(),
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),

    };
    let post = Post {
        title: "my cool post".to_string(),
    };

    let query = Person::find().build();

    println!("{}", query);
}
