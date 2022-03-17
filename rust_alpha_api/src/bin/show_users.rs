extern crate rust_alpha_api;
extern crate diesel;

use self::rust_alpha_api::establish_connection;
//use self::models::*;
use self::diesel::prelude::*;
use rust_alpha_api::models::Users;

fn main() {
    use rust_alpha_api::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<Users>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{:?}", user.first_name);
        println!("----------\n");
        println!("{:?}", user.last_name);
    }
}