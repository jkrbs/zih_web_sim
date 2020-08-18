#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use crate::models::{User, NewUser, UserForm, UpdateNameForm};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use crate::schema::users::dsl::users;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_new_user(u: UserForm) -> (String, String) {
    println!("{}:{}", u.first_name, u.last_name);
    let mut rng = thread_rng();

    let password = rng
        .sample_iter(&Alphanumeric)
        .take(30)
        .collect();
    let id: u32 = rng.gen();

    let conn = establish_connection();
    let new_user = NewUser{
        first_name: &u.first_name,
        last_name: &u.last_name,
        birthdate: &u.birthdate,
        address: &u.address,
        password: &password,
        id: &id.to_string()
    };

    diesel::insert_into(users)
    .values(&new_user)
    .execute(&conn)
    .expect("Error saving new post");

    (password, id.to_string())  
}

pub fn get_user_list() -> Vec<User> {
    use crate::schema::users::dsl::*;

    let conn = establish_connection();
    users.load(&conn).expect("Error loading posts")
}

pub fn get_user(uid: String) -> Vec<User> {
    use crate::schema::users::dsl::*;

    let conn = establish_connection();
    users.find(uid).load(&conn).expect("err")

}

pub fn update_name(update: UpdateNameForm) {
    use crate::schema::users::dsl::*;

    let conn = establish_connection();
    let uid = update.uid;
    diesel::update(users.find(&uid))
    .set(last_name.eq(update.last_name))
    .execute(&conn)
    .expect("error while updating username");

    diesel::update(users.find(&uid))
    .set(first_name.eq(update.first_name))
    .execute(&conn)
    .expect("error while updating username");
}