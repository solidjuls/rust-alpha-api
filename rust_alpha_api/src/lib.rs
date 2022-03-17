#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use crate::models::NewUsers;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_users<'u>(conn: &MysqlConnection) {
    use schema::users;

    let new_users = NewUsers {
        name: "Juli",
        email: "juli@gmail.com",
        password: "verysecure",
        remember_token: "token",
        first_name: "Juli",
        last_name: "Arnalot",
    };

    diesel::insert_into(users::table)
        .values(&new_users)
        .execute(conn)
        .expect("Error inserting new user");
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
