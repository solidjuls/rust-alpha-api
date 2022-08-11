// extern crate rust_alpha_api;
extern crate diesel;

use crate::establish_connection;
use crate::create_users;
use self::diesel::prelude::*;
use crate::models::Users;

pub fn write_users() {
    use crate::schema::users::dsl::*;
    let connection = establish_connection();

    // println!("What would you like your title to be?");
    // let mut title = String::new();
    // stdin().read_line(&mut title).unwrap();
    // let title = &title[..(title.len() - 1)]; // Drop the newline character
    // println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    // let mut body = String::new();
    // stdin().read_to_string(&mut body).unwrap();

    let user = create_users(&connection);
    user
    // println!("\nSaved draft {} with id {}", title, user.);
}
