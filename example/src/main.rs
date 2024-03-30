use surreal_client::table::Table;
use surreal_client_macro::Table;


#[derive(Table)]
struct Person {
    name: String,
}

#[derive(Table)]
struct Post {
    title: String,
}

fn main() {
    let person = Person {
        name: "John Doe".to_string(),
    };
    let post = Post {
        title: "my cool post".to_string(),
    };

    let persons = Person::find().build();


    surreal_client::field::Field {
        attrs
    }

    println!("{}", persons);
}
