// extern crate rust_alpha_api;
extern crate diesel;

// use model::establish_connection;
use crate::establish_connection;
//use self::models::*;
use self::diesel::prelude::*;
use crate::models::Users;

pub fn show_users() -> Vec<Users> {
    use crate::schema::users::dsl::*;

    let connection = establish_connection();
    let results = users
        .limit(5)
        .load::<Users>(&connection)
        .expect("Error loading posts");
    results
    // println!("Displaying {} users", results.len());
    // for user in results {
    //     println!("{:?}", user.first_name);
    //     println!("----------\n");
    //     println!("{:?}", user.last_name);
    // }
}