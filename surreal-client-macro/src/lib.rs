// use macro_state::{proc_init_state, proc_read_state};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Table)]
pub fn table(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    // Extract fields from the input struct
    let fields: Vec<surreal_client::field::Field> = if let syn::Data::Struct(data) = input.data {
        data.fields
            .iter()
            .map(surreal_client::field::Field::from)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let static_fields = fields.iter().map(surreal_client::field::StaticField::from);

    let expanded = quote! {
        impl<'a> surreal_client::table::Table<'a> for #name {
            fn name() -> &'a str {
                static NAME: &str = stringify!(#name);
                NAME
            }

            fn fields() -> &'a [surreal_client::field::StaticField<'a>] {
                static FIELDS: &[surreal_client::field::StaticField] = &[#(#static_fields),*];
                FIELDS
            }

            fn find() -> surreal_client::query::Query<'a, Self> {
                surreal_client::query::Query::<'a, Self>::new()
            }
        }
    };

    TokenStream::from(expanded)
}

// comes from crate::table::Table

// pub trait Table<'a>: Sized {
//     fn name() -> &'a str;
//     fn fields() -> &'a [Field<'a>];
//     fn find() -> Query<'a, Self>;
// }

// pub struct User {
//     name: String,
// }

// impl<'a> Table<'a> for User {
//     fn name() -> &'a str {
//         "User"
//     }

//     fn fields() -> &'a [Field<'a>] {
//         &[Field {
//             name: "name",
//             field_type: FieldType::String,
//         }]
//     }

//     fn find() -> Query<'a, Self> {
//         Query::<'a, Self>::new()
//     }
// }

// proc_init_state("blah", &name.to_string()).expect("Could not initialize state");
// println!("{}", proc_read_state("blah").expect("Could not read state"));

// create an empty txt file named "table.txt"
// std::fs::write("table.txt", "").expect("Unable to write file");

// Track Schema Versions: You need to keep track of the version of your schema. This could be done by maintaining a version number in a dedicated table in your database. Each time you perform a migration, you increment this version number.

// Generate Migration Files: Each time you make changes to your Rust structs, you generate a new migration file. This file should contain the SQL commands necessary to transform the database from the previous version to the current version. The filename of the migration file could include the version number for easy reference.

// Apply Migrations: When your application starts, it checks the current database version against the migration files. If there are migration files for higher versions, it applies these migrations in order, updating the database version each time.

// Rollback Mechanism: It's also a good idea to include a rollback mechanism. Each migration file could include the SQL commands to rollback the changes, in case something goes wrong.
