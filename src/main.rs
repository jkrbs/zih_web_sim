#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::LenientForm;
use zih_web_sim::{create_new_user, get_user, get_user_list};
use zih_web_sim::models::UserForm;

use std::io;
use rocket::response::{NamedFile};
use rocket_contrib::templates::Template;


#[macro_use] extern crate rocket;


use zih_web_sim::models::User;
#[derive(serde::Serialize)]
struct Users<'a> {
    users: &'a Vec<User> 
}

#[post("/api/matriculate", data="<user>")]
fn matriculate(user: LenientForm<UserForm>) -> String {
    let e = create_new_user(user.into_inner());
    
    format!("{}, {}", e.0, e.1)
}


#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/new_user")]
pub fn new_user() -> io::Result<NamedFile> {
    NamedFile::open("static/new_user.html")
}

#[get("/show_user/<uid>")]
pub fn show_user(uid: String) -> Template {
    let user = &get_user(uid)[0];
    Template::render("show_user", user)
}

#[get("/list_users")]
pub fn list_users() -> Template {
    let user = &get_user_list();
    Template::render("list_users", Users{users: user})
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, new_user, matriculate, show_user, list_users])
    .attach(Template::fairing())
    .launch();
}