extern crate rust_alpha_api;
extern crate diesel;

use self::rust_alpha_api::*;
//use self::models::*;
use self::diesel::prelude::*;
use rust_alpha_api::models::Users;

fn main() {
    use rust_alpha_api::schema::users::dsl::*;
    let connection = establish_connection();

    // println!("What would you like your title to be?");
    // let mut title = String::new();
    // stdin().read_line(&mut title).unwrap();
    // let title = &title[..(title.len() - 1)]; // Drop the newline character
    // println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    // let mut body = String::new();
    // stdin().read_to_string(&mut body).unwrap();

    let user = create_users(&connection);
    // println!("\nSaved draft {} with id {}", title, user.);
}

#[cfg(not(windows))]
const EOF: &'static str = "CTRL+D";

#[cfg(windows)]
const EOF: &'static str = "CTRL+Z";