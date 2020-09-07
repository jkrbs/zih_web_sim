#![feature(proc_macro_hygiene, decl_macro)]

use rocket::request::LenientForm;
use zih_web_sim::{create_new_user, get_user, get_user_list, update_name};
use zih_web_sim::models::*;

use std::io;
use rocket::response::{NamedFile};
use rocket_contrib::templates::Template;


#[macro_use] extern crate rocket;


use zih_web_sim::models::User;
#[derive(serde::Serialize)]
struct Users<'a> {
    users: &'a Vec<User> 
}

#[derive(serde::Serialize)]
struct TErr<'a> {
    error: &'a String
}

#[post("/api/matriculate", data="<user>")]
fn matriculate(user: LenientForm<UserForm>) -> String {
    let e = create_new_user(user.into_inner());
    
    format!("password:{}, id:{}", e.0, e.1)
}

#[post("/api/update/name", data="<update>")]
fn updatename(update: LenientForm<UpdateNameForm>) -> String {
    update_name(update.into_inner());
    "ok".to_string()
}


#[get("/")]
pub fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/new_user")]
pub fn new_user() -> io::Result<NamedFile> {
    NamedFile::open("static/new_user.html")
}

#[get("/show_user?<uid>&<password>")]
pub fn show_user(uid: String, password: String) -> Template {
    let user = &get_user(uid)[0];
    if user.password == password  {
        Template::render("show_user", user)
    } else {
        Template::render("error", TErr{error: &"wrong password".to_string()} )
    }
}

#[get("/update_user_name/<uid>")]
pub fn update_username(uid: String) -> Template {
    let user = &get_user(uid)[0];
    Template::render("update_user_name", user)
}

#[get("/list_users")]
pub fn list_users() -> Template {
    let user = &get_user_list();
    Template::render("list_users", Users{users: user})
}

fn main() {
    rocket::ignite()
    .mount("/", routes![index, new_user, matriculate, show_user, list_users, updatename, update_username])
    .attach(Template::fairing())
    .launch();
}