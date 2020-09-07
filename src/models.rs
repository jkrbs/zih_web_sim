use rocket::request::FromForm;

#[derive(FromForm)]
pub struct UserForm {
    pub first_name: String,
    pub last_name: String,
    pub birthdate: String,
    pub address: String,
    pub password: Option<String>,
    pub id: Option<String>
}

#[derive(FromForm)]
pub struct UpdateNameForm {
    pub uid: String,
    pub first_name: String,
    pub last_name: String
}

#[derive(Queryable, serde::Serialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub id: String,
    pub address: String,
    pub password: String,
    pub birthdate: String
}

use crate::schema::users;
#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub first_name: &'a String,
    pub last_name: &'a String,
    pub birthdate: &'a String,
    pub address: &'a String,
    pub password: &'a String,
    pub id: &'a String
}